mod analytics;
mod content;
mod system;
mod utils;

use analytics::{get_post_views, record_post_view};
use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use content::loader::load_content_from_dir;
use content::models::{ContentItem, ContentKind};
use sqlx::SqlitePool;
use std::{path::Path as StdPath, sync::Arc};
use system::models::{SystemInfo, SystemMetrics};
use system::monitor::{get_system_info, get_system_metrics};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

#[derive(Clone)]
struct AppState {
    writing: Arc<Vec<ContentItem>>,
    projects: Arc<Vec<ContentItem>>,
    db: SqlitePool,
}

async fn system_info_handler() -> Json<SystemInfo> {
    Json(get_system_info())
}

async fn system_metrics_handler() -> Json<SystemMetrics> {
    Json(get_system_metrics())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let writing = load_content_from_dir(StdPath::new("./content/writing"), ContentKind::Writing)
        .expect("Failed to load writing posts");

    let projects = load_content_from_dir(StdPath::new("./content/projects"), ContentKind::Project)
        .expect("Failed to load projects");

    let pool = SqlitePool::connect("sqlite://./db/analytics.db")
        .await
        .expect("Failed to connect to SQLite");

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

    let app = Router::new()
        // Writing
        .route("/api/writing", get(list_writing))
        .route("/api/writing/{slug}", get(get_writing))
        // Projects
        .route("/api/projects", get(list_projects))
        .route("/api/projects/{slug}", get(get_project))
        // Analytics
        .route("/api/{post_type}/{id}/view", post(record_post_view))
        .route("/api/{post_type}/{id}/views", get(get_post_views))
        // System
        .route("/api/system/info", get(system_info_handler))
        .route("/api/system/metrics", get(system_metrics_handler))
        // Static assets
        .nest_service("/assets", ServeDir::new("./content"))
        .with_state(state)
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

async fn list_writing(State(state): State<AppState>) -> Json<Vec<ContentItem>> {
    Json(state.writing.as_ref().clone())
}

async fn get_writing(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ContentItem>, (StatusCode, &'static str)> {
    state
        .writing
        .iter()
        .find(|p| p.slug == slug)
        .cloned()
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, "Post not found"))
}

async fn list_projects(State(state): State<AppState>) -> Json<Vec<ContentItem>> {
    Json(state.projects.as_ref().clone())
}

async fn get_project(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ContentItem>, (StatusCode, &'static str)> {
    state
        .projects
        .iter()
        .find(|p| p.slug == slug)
        .cloned()
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, "Project not found"))
}
