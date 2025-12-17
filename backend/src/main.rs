mod content;
mod utils;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use content::loader::load_content_from_dir;
use content::models::ContentItem;
use std::{path::Path as StdPath, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load posts
    let posts: Vec<ContentItem> =
        load_content_from_dir(StdPath::new("./content/posts")).expect("Failed to load posts");
    let posts = Arc::new(posts);
    info!("Loaded {} posts", posts.len());

    // Build router
    let app = Router::new()
        .route("/api/posts", get(list_posts))
        .route("/api/posts/{slug}", get(get_post))
        .with_state(posts.clone())
        .nest_service("/assets", ServeDir::new("./content"));

    let addr = "127.0.0.1:3001";
    info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_posts(State(posts): State<Arc<Vec<ContentItem>>>) -> Json<Vec<ContentItem>> {
    Json(posts.to_vec())
}

async fn get_post(
    State(posts): State<Arc<Vec<ContentItem>>>,
    Path(slug): Path<String>,
) -> Result<Json<ContentItem>, (StatusCode, &'static str)> {
    match posts.iter().find(|p| p.slug == slug) {
        Some(post) => Ok(Json(post.clone())),
        None => Err((StatusCode::NOT_FOUND, "Post not found")),
    }
}
