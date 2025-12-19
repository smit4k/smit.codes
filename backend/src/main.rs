mod content;
mod utils;

use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    routing::get,
    Json, Router,
};
use content::loader::load_content_from_dir;
use content::models::{ContentItem, ContentKind};
use std::{path::Path as StdPath, sync::Arc};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

#[derive(Clone)]
struct AppState {
    writing: Arc<Vec<ContentItem>>,
    projects: Arc<Vec<ContentItem>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let writing = load_content_from_dir(StdPath::new("./content/writing"), ContentKind::Writing)
        .expect("Failed to load writing posts");

    let projects = load_content_from_dir(StdPath::new("./content/projects"), ContentKind::Project)
        .expect("Failed to load projects");

    let state = AppState {
        writing: Arc::new(writing),
        projects: Arc::new(projects),
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
        // Static assets (images, etc.)
        .nest_service("/assets", ServeDir::new("./content"))
        .with_state(state)
        .layer(cors);

    let addr = "127.0.0.1:3001";
    info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
