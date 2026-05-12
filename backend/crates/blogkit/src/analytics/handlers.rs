use super::views::{
    get_page_view_count, get_site_stats, record_page_view, PageViewCount, SiteStats,
};
use super::views::{get_view_count, record_view, ViewCount};
use axum::{
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
};
use tracing::error;

const MAX_USER_AGENT_LEN: usize = 256;
const MAX_PAGE_PATH_LEN: usize = 256;

#[derive(Clone)]
pub struct AnalyticsState {
    pub db: SqlitePool,
    post_collections: Arc<HashMap<String, PostCollection>>,
}

#[derive(Clone)]
struct PostCollection {
    stored_type: String,
    slugs: HashSet<String>,
}

impl AnalyticsState {
    pub fn new(db: SqlitePool) -> Self {
        Self {
            db,
            post_collections: Arc::new(HashMap::new()),
        }
    }

    pub fn with_post_collection<I, S>(
        mut self,
        route_type: impl Into<String>,
        stored_type: impl Into<String>,
        slugs: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut post_collections = (*self.post_collections).clone();
        post_collections.insert(
            route_type.into(),
            PostCollection {
                stored_type: stored_type.into(),
                slugs: slugs.into_iter().map(Into::into).collect(),
            },
        );
        self.post_collections = Arc::new(post_collections);
        self
    }

    fn resolve_post_type(&self, post_type: &str, slug: &str) -> Option<&str> {
        let collection = self.post_collections.get(post_type)?;
        if collection.slugs.contains(slug) {
            Some(collection.stored_type.as_str())
        } else {
            None
        }
    }
}

pub fn analytics_router(state: AnalyticsState) -> Router {
    analytics_read_router(state.clone()).merge(analytics_write_router(state))
}

pub fn analytics_read_router(state: AnalyticsState) -> Router {
    Router::new()
        .route("/api/views", get(get_all_page_views))
        .route("/api/views/{*page_path}", get(get_page_views))
        .route("/api/{post_type}/{slug}/views", get(get_post_views))
        .with_state(state)
}

pub fn analytics_write_router(state: AnalyticsState) -> Router {
    Router::new()
        .route("/api/views/track", post(track_page_view))
        .route("/api/{post_type}/{slug}/view", post(record_post_view))
        .with_state(state)
}

/// Use the socket address directly unless a trusted proxy layer is introduced.
fn extract_ip(_headers: &HeaderMap, addr: &SocketAddr) -> IpAddr {
    addr.ip()
}

fn anonymize_ip(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(v4) => {
            let octets = v4.octets();
            Ipv4Addr::new(octets[0], octets[1], octets[2], 0).to_string()
        }
        IpAddr::V6(v6) => {
            let segments = v6.segments();
            Ipv6Addr::new(
                segments[0],
                segments[1],
                segments[2],
                segments[3],
                0,
                0,
                0,
                0,
            )
            .to_string()
        }
    }
}

fn sanitize_user_agent(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get("user-agent")?.to_str().ok()?.trim();
    if raw.is_empty() {
        return None;
    }

    let truncated: String = raw.chars().take(MAX_USER_AGENT_LEN).collect();
    Some(
        truncated
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>(),
    )
}

fn normalize_page_path(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_PAGE_PATH_LEN {
        return None;
    }

    let without_query = trimmed.split(['?', '#']).next().unwrap_or(trimmed).trim();

    if without_query.is_empty()
        || !without_query.starts_with('/')
        || without_query.contains('\\')
        || without_query.contains("..")
        || without_query.contains("//")
        || without_query.chars().any(|c| c.is_control())
    {
        return None;
    }

    if !without_query
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '-' | '_' | '.'))
    {
        return None;
    }

    Some(without_query.to_string())
}

/// POST /api/:post_type/:slug/view
/// Records a view for a post/project
pub async fn record_post_view(
    Path((post_type, slug)): Path<(String, String)>,
    State(state): State<AnalyticsState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    let pool = &state.db;

    if !state.post_collections.contains_key(&post_type) {
        error!("Invalid post_type: {}", post_type);
        return Err(StatusCode::BAD_REQUEST);
    }

    let stored_type = match state.resolve_post_type(&post_type, &slug) {
        Some(stored_type) => stored_type,
        None => {
            error!("Post not found: {}/{}", post_type, slug);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let viewer_ip = extract_ip(&headers, &addr);
    let ip_string = anonymize_ip(viewer_ip);
    let user_agent = sanitize_user_agent(&headers);

    if let Err(e) = record_view(pool, &slug, stored_type, Some(ip_string), user_agent).await {
        error!(
            "Failed to record view for {}: {} (DB Error: {:?})",
            slug, stored_type, e
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::CREATED)
}

/// GET /api/:post_type/:slug/views
/// Returns view count for a post/project
pub async fn get_post_views(
    Path((_post_type, slug)): Path<(String, String)>,
    State(state): State<AnalyticsState>,
) -> Result<Json<ViewCount>, StatusCode> {
    let pool = &state.db;

    match get_view_count(pool, &slug).await {
        Ok(views) => Ok(Json(views)),
        Err(e) => {
            error!("Failed to get view count for {}: {:?}", slug, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// --- Page view tracking (all pages) ---

#[derive(Debug, Deserialize)]
pub struct TrackPageViewRequest {
    pub page_path: String,
}

/// POST /api/views/track
/// Records a page view for any page
pub async fn track_page_view(
    State(state): State<AnalyticsState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<TrackPageViewRequest>,
) -> Result<StatusCode, StatusCode> {
    let pool = &state.db;

    let Some(page_path) = normalize_page_path(&body.page_path) else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let viewer_ip = extract_ip(&headers, &addr);
    let ip_string = anonymize_ip(viewer_ip);
    let user_agent = sanitize_user_agent(&headers);

    if let Err(e) = record_page_view(pool, &page_path, Some(ip_string), user_agent).await {
        error!("Failed to record page view for {}: {:?}", page_path, e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::CREATED)
}

/// GET /api/views
/// Returns site-wide view stats with per-page breakdown
pub async fn get_all_page_views(
    State(state): State<AnalyticsState>,
) -> Result<Json<SiteStats>, StatusCode> {
    let pool = &state.db;

    match get_site_stats(pool).await {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            error!("Failed to get site stats: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/views/:page_path
/// Returns view count for a specific page
pub async fn get_page_views(
    Path(page_path): Path<String>,
    State(state): State<AnalyticsState>,
) -> Result<Json<PageViewCount>, StatusCode> {
    let pool = &state.db;
    let decoded_path = format!("/{}", page_path);

    match get_page_view_count(pool, &decoded_path).await {
        Ok(views) => Ok(Json(views)),
        Err(e) => {
            error!(
                "Failed to get page view count for {}: {:?}",
                decoded_path, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
