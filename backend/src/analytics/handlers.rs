use super::views::{
    get_page_view_count, get_site_stats, record_page_view, PageViewCount, SiteStats,
};
use super::views::{get_view_count, record_view, ViewCount};
use crate::AppState;
use axum::{
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use tracing::error;

const MAX_USER_AGENT_LEN: usize = 256;
const MAX_PAGE_PATH_LEN: usize = 256;

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
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    let pool = &state.db;

    if post_type != "writing" && post_type != "projects" && post_type != "photos" {
        error!("Invalid post_type: {}", post_type);
        return Err(StatusCode::BAD_REQUEST);
    }

    let post_slug = match post_type.as_str() {
        "writing" => state
            .writing
            .iter()
            .find(|p| p.slug == slug)
            .map(|p| p.slug.clone()),
        "projects" => state
            .projects
            .iter()
            .find(|p| p.slug == slug)
            .map(|p| p.slug.clone()),
        "photos" => state
            .photos
            .iter()
            .find(|p| p.slug == slug)
            .map(|p| p.slug.clone()),
        _ => None,
    };

    let post_slug = match post_slug {
        Some(s) => s,
        None => {
            error!("Post not found: {}/{}", post_type, slug);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let viewer_ip = extract_ip(&headers, &addr);
    let ip_string = anonymize_ip(viewer_ip);
    let user_agent = sanitize_user_agent(&headers);

    let type_singular = post_type.trim_end_matches('s');

    if let Err(e) = record_view(pool, &post_slug, type_singular, Some(ip_string), user_agent).await
    {
        error!(
            "Failed to record view for {}: {} (DB Error: {:?})",
            post_slug, type_singular, e
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::CREATED)
}

/// GET /api/:post_type/:slug/views
/// Returns view count for a post/project
pub async fn get_post_views(
    Path((_post_type, slug)): Path<(String, String)>,
    State(state): State<AppState>,
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
    State(state): State<AppState>,
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
    State(state): State<AppState>,
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
    State(state): State<AppState>,
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
