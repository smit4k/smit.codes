mod analytics;
mod content;
mod system;
mod utils;

use analytics::create_pool;
use analytics::{get_all_page_views, get_page_views, track_page_view};
use analytics::{get_post_views, record_post_view};
use axum::{
    extract::DefaultBodyLimit,
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use content::loader::load_content_from_dir;
use content::models::{ContentItem, ContentKind};
use sqlx::SqlitePool;
use std::{path::Path as StdPath, sync::Arc};
use system::models::{SystemInfo, SystemMetrics};
use system::monitor::{get_system_info, get_system_metrics};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

#[derive(Clone)]
struct AppState {
    writing: Arc<Vec<ContentItem>>,
    projects: Arc<Vec<ContentItem>>,
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let writing = load_content_from_dir(StdPath::new("./content/writing"), ContentKind::Writing)
        .expect("Failed to load writing posts");

    let projects = load_content_from_dir(StdPath::new("./content/projects"), ContentKind::Project)
        .expect("Failed to load projects");

    let pool = create_pool().await.expect("Failed to initialize database");

    let state = AppState {
        writing: Arc::new(writing),
        projects: Arc::new(projects),
        db: pool,
    };

    info!(
        "Loaded {} writing posts, {} projects",
        state.writing.len(),
        state.projects.len()
    );

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any);

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(25)
            .finish()
            .unwrap(),
    );

    let app = Router::new()
        // Writing
        .route("/api/writing", get(list_writing))
        .route("/api/writing/{slug}", get(get_writing))
        // Projects
        .route("/api/projects", get(list_projects))
        .route("/api/projects/{slug}", get(get_project))
        // Analytics - page views (before post views to avoid route conflicts)
        .route("/api/views/track", post(track_page_view))
        .route("/api/views", get(get_all_page_views))
        .route("/api/views/{*page_path}", get(get_page_views))
        // Analytics - post views
        .route("/api/{post_type}/{slug}/view", post(record_post_view))
        .route("/api/{post_type}/{slug}/views", get(get_post_views))
        // System
        .route("/api/system/info", get(system_info_handler))
        .route("/api/system/metrics", get(system_metrics_handler))
        // Static assets
        .nest_service("/assets", ServeDir::new("./content"))
        .layer(DefaultBodyLimit::max(1024))
        .with_state(state)
        .layer(GovernorLayer::new(governor_conf))
        .layer(cors);

    let addr = "127.0.0.1:3001";
    info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}

// --- Handlers ---

async fn system_info_handler() -> Json<SystemInfo> {
    Json(get_system_info())
}

async fn system_metrics_handler() -> Json<SystemMetrics> {
    Json(get_system_metrics())
}

async fn list_writing(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let body = serde_json::to_vec(state.writing.as_ref()).unwrap();
    let etag = utils::cache::etag_from_bytes(&body);

    if let Some(if_none_match) = headers.get("If-None-Match") {
        if if_none_match == etag {
            return StatusCode::NOT_MODIFIED.into_response();
        }
    }

    let response_headers = utils::cache::default_cache_headers(etag);
    (response_headers, body).into_response()
}

async fn get_writing(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Response, (StatusCode, &'static str)> {
    let post = state
        .writing
        .iter()
        .find(|p| p.slug == slug)
        .cloned()
        .ok_or((StatusCode::NOT_FOUND, "Post not found"))?;

    let body = serde_json::to_vec(&post).unwrap();
    let etag = utils::cache::etag_from_bytes(&body);

    if let Some(if_none_match) = headers.get("If-None-Match") {
        if if_none_match == etag {
            return Ok(StatusCode::NOT_MODIFIED.into_response());
        }
    }

    let response_headers = utils::cache::default_cache_headers(etag);

    Ok((response_headers, body).into_response())
}

async fn list_projects(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let body = serde_json::to_vec(state.projects.as_ref()).unwrap();
    let etag = utils::cache::etag_from_bytes(&body);

    if let Some(if_none_match) = headers.get("If-None-Match") {
        if if_none_match == etag {
            return StatusCode::NOT_MODIFIED.into_response();
        }
    }

    let response_headers = utils::cache::default_cache_headers(etag);
    (response_headers, body).into_response()
}

async fn get_project(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Response, (StatusCode, &'static str)> {
    let project = state
        .projects
        .iter()
        .find(|p| p.slug == slug)
        .cloned()
        .ok_or((StatusCode::NOT_FOUND, "Project not found"))?;

    let body = serde_json::to_vec(&project).unwrap();
    let etag = utils::cache::etag_from_bytes(&body);

    if let Some(if_none_match) = headers.get("If-None-Match") {
        if if_none_match == etag {
            return Ok(StatusCode::NOT_MODIFIED.into_response());
        }
    }

    let response_headers = utils::cache::default_cache_headers(etag);
    Ok((response_headers, body).into_response())
}
