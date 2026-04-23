mod analytics;
mod content;
mod system;
mod utils;

use analytics::create_pool;
use analytics::{get_all_page_views, get_page_views, track_page_view};
use analytics::{get_post_views, record_post_view};
use axum::{
    body::Bytes,
    extract::DefaultBodyLimit,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use content::loader::{load_content_from_dir, load_photo_posts_from_dir};
use content::models::{ContentItem, ContentKind, PhotoPost};
use sqlx::SqlitePool;
use std::{collections::HashMap, env, path::Path as StdPath, sync::Arc};
use system::models::{SystemInfo, SystemMetrics};
use system::monitor::{get_system_info, get_system_metrics};
use tower_governor::{
    governor::GovernorConfigBuilder,
    key_extractor::{PeerIpKeyExtractor, SmartIpKeyExtractor},
    GovernorLayer,
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

#[derive(Clone)]
struct AppState {
    writing: Arc<Vec<ContentItem>>,
    projects: Arc<Vec<ContentItem>>,
    photos: Arc<Vec<PhotoPost>>,
    writing_list: Arc<CachedPayload>,
    projects_list: Arc<CachedPayload>,
    photos_list: Arc<CachedPayload>,
    writing_by_slug: Arc<HashMap<String, CachedPayload>>,
    projects_by_slug: Arc<HashMap<String, CachedPayload>>,
    photos_by_slug: Arc<HashMap<String, CachedPayload>>,
    db: SqlitePool,
}

#[derive(Clone)]
struct CachedPayload {
    etag: HeaderValue,
    body: Bytes,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let writing = load_content_from_dir(StdPath::new("./content/writing"), ContentKind::Writing)
        .expect("Failed to load writing posts");

    let projects = load_content_from_dir(StdPath::new("./content/projects"), ContentKind::Project)
        .expect("Failed to load projects");
    let photos = load_photo_posts_from_dir(StdPath::new("./content/photos"))
        .expect("Failed to load photo posts");
    let writing_list = Arc::new(cached_payload(&writing));
    let projects_list = Arc::new(cached_payload(&projects));
    let photos_list = Arc::new(cached_payload(&photos));
    let writing_by_slug = Arc::new(cache_content_items_by_slug(&writing));
    let projects_by_slug = Arc::new(cache_content_items_by_slug(&projects));
    let photos_by_slug = Arc::new(cache_photo_posts_by_slug(&photos));

    let pool = create_pool().await.expect("Failed to initialize database");

    let state = AppState {
        writing: Arc::new(writing),
        projects: Arc::new(projects),
        photos: Arc::new(photos),
        writing_list,
        projects_list,
        photos_list,
        writing_by_slug,
        projects_by_slug,
        photos_by_slug,
        db: pool,
    };

    info!(
        "Loaded {} writing posts, {} projects, {} photo posts",
        state.writing.len(),
        state.projects.len(),
        state.photos.len()
    );

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any);

    let content_api = Router::new()
        .route("/api/writing", get(list_writing))
        .route("/api/writing/{slug}", get(get_writing))
        .route("/api/projects", get(list_projects))
        .route("/api/projects/{slug}", get(get_project))
        .route("/api/photos", get(list_photos))
        .route("/api/photos/{slug}", get(get_photo));

    let analytics_read_api = Router::new()
        .route("/api/views", get(get_all_page_views))
        .route("/api/views/{*page_path}", get(get_page_views))
        .route("/api/{post_type}/{slug}/views", get(get_post_views));

    let analytics_write_api = Router::new()
        .route("/api/views/track", post(track_page_view))
        .route("/api/{post_type}/{slug}/view", post(record_post_view));

    let system_api = Router::new()
        .route("/api/system/info", get(system_info_handler))
        .route("/api/system/metrics", get(system_metrics_handler));

    let rate_limit_enabled = env_flag("ENABLE_RATE_LIMIT", true);
    let (content_api, analytics_read_api, analytics_write_api, system_api) = if rate_limit_enabled {
        let trust_proxy_headers = env_flag("RATE_LIMIT_TRUST_PROXY_HEADERS", true);
        let content_api = apply_rate_limit(
            content_api,
            "content-read",
            60,
            180,
            trust_proxy_headers,
            vec![Method::GET],
        );
        let analytics_read_api = apply_rate_limit(
            analytics_read_api,
            "analytics-read",
            10,
            30,
            trust_proxy_headers,
            vec![Method::GET],
        );
        let analytics_write_api = apply_rate_limit(
            analytics_write_api,
            "analytics-write",
            5,
            20,
            trust_proxy_headers,
            vec![Method::POST],
        );
        let system_api = apply_rate_limit(
            system_api,
            "system",
            2,
            6,
            trust_proxy_headers,
            vec![Method::GET],
        );

        info!(
            "Rate limiting enabled with endpoint-specific policies (trust_proxy_headers={})",
            trust_proxy_headers
        );
        (
            content_api,
            analytics_read_api,
            analytics_write_api,
            system_api,
        )
    } else {
        info!("Rate limiting disabled");
        (
            content_api,
            analytics_read_api,
            analytics_write_api,
            system_api,
        )
    };

    let app = Router::new()
        .merge(content_api)
        .merge(analytics_read_api)
        .merge(analytics_write_api)
        .merge(system_api)
        .nest_service("/assets", ServeDir::new("./content"))
        .layer(DefaultBodyLimit::max(1024))
        .with_state(state)
        .layer(cors);

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3001".into());
    info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}

fn env_flag(name: &str, default: bool) -> bool {
    match env::var(name) {
        Ok(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => default,
    }
}

fn apply_rate_limit(
    router: Router<AppState>,
    policy_name: &str,
    per_second: u64,
    burst_size: u32,
    trust_proxy_headers: bool,
    methods: Vec<Method>,
) -> Router<AppState> {
    if trust_proxy_headers {
        let mut builder = GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .use_headers();
        builder
            .per_second(rate_limit_per_second(policy_name, per_second))
            .burst_size(rate_limit_burst_size(policy_name, burst_size))
            .methods(methods);
        router.layer(GovernorLayer::new(Arc::new(
            builder.finish().expect("invalid rate limit config"),
        )))
    } else {
        let mut builder = GovernorConfigBuilder::default()
            .key_extractor(PeerIpKeyExtractor)
            .use_headers();
        builder
            .per_second(rate_limit_per_second(policy_name, per_second))
            .burst_size(rate_limit_burst_size(policy_name, burst_size))
            .methods(methods);
        router.layer(GovernorLayer::new(Arc::new(
            builder.finish().expect("invalid rate limit config"),
        )))
    }
}

fn rate_limit_per_second(policy_name: &str, default: u64) -> u64 {
    env::var(rate_limit_env_name(policy_name, "PER_SECOND"))
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn rate_limit_burst_size(policy_name: &str, default: u32) -> u32 {
    env::var(rate_limit_env_name(policy_name, "BURST"))
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn rate_limit_env_name(policy_name: &str, suffix: &str) -> String {
    format!(
        "RATE_LIMIT_{}_{}",
        policy_name.replace('-', "_").to_ascii_uppercase(),
        suffix
    )
}

// --- Handlers ---

async fn system_info_handler() -> Json<SystemInfo> {
    Json(get_system_info())
}

async fn system_metrics_handler() -> Json<SystemMetrics> {
    Json(get_system_metrics())
}

async fn list_writing(State(state): State<AppState>, headers: HeaderMap) -> Response {
    respond_cached(&headers, state.writing_list.as_ref())
}

async fn get_writing(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Response, (StatusCode, &'static str)> {
    let post = state
        .writing_by_slug
        .get(&slug)
        .ok_or((StatusCode::NOT_FOUND, "Post not found"))?;
    Ok(respond_cached(&headers, post))
}

async fn list_projects(State(state): State<AppState>, headers: HeaderMap) -> Response {
    respond_cached(&headers, state.projects_list.as_ref())
}

async fn get_project(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Response, (StatusCode, &'static str)> {
    let project = state
        .projects_by_slug
        .get(&slug)
        .ok_or((StatusCode::NOT_FOUND, "Project not found"))?;
    Ok(respond_cached(&headers, project))
}

async fn list_photos(State(state): State<AppState>, headers: HeaderMap) -> Response {
    respond_cached(&headers, state.photos_list.as_ref())
}

async fn get_photo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Response, (StatusCode, &'static str)> {
    let photo = state
        .photos_by_slug
        .get(&slug)
        .ok_or((StatusCode::NOT_FOUND, "Photo post not found"))?;
    Ok(respond_cached(&headers, photo))
}

fn cached_payload<T: serde::Serialize>(value: &T) -> CachedPayload {
    let body = Bytes::from(serde_json::to_vec(value).expect("Failed to serialize cached payload"));
    let etag = utils::cache::etag_from_bytes(&body);

    CachedPayload { etag, body }
}

fn cache_content_items_by_slug(items: &[ContentItem]) -> HashMap<String, CachedPayload> {
    items
        .iter()
        .map(|item| (item.slug.clone(), cached_payload(item)))
        .collect()
}

fn cache_photo_posts_by_slug(items: &[PhotoPost]) -> HashMap<String, CachedPayload> {
    items
        .iter()
        .map(|item| (item.slug.clone(), cached_payload(item)))
        .collect()
}

fn respond_cached(headers: &HeaderMap, payload: &CachedPayload) -> Response {
    if let Some(if_none_match) = headers.get("If-None-Match") {
        if if_none_match == payload.etag {
            return StatusCode::NOT_MODIFIED.into_response();
        }
    }

    let response_headers = utils::cache::default_cache_headers(payload.etag.clone());
    (response_headers, payload.body.clone()).into_response()
}
