use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method},
    Router,
};
use blogkit::{
    analytics::{analytics_read_router, analytics_write_router, create_pool, AnalyticsState},
    axum::{apply_rate_limit, content_collection_router, system_router, CachedCollection},
    content::{
        loader::{load_content_from_dir, load_photo_posts_from_dir},
        models::ContentKind,
    },
};
use std::{env, path::Path as StdPath};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let writing = load_content_from_dir(StdPath::new("./content/writing"), ContentKind::Writing)
        .expect("Failed to load writing posts");
    let projects = load_content_from_dir(StdPath::new("./content/projects"), ContentKind::Project)
        .expect("Failed to load projects");
    let photos = load_photo_posts_from_dir(StdPath::new("./content/photos"))
        .expect("Failed to load photo posts");

    info!(
        "Loaded {} writing posts, {} projects, {} photo posts",
        writing.len(),
        projects.len(),
        photos.len()
    );

    let analytics_state =
        AnalyticsState::new(create_pool().await.expect("Failed to initialize database"))
            .with_post_collection(
                "writing",
                "writing",
                writing.iter().map(|post| post.slug.clone()),
            )
            .with_post_collection(
                "projects",
                "project",
                projects.iter().map(|project| project.slug.clone()),
            )
            .with_post_collection(
                "photos",
                "photo",
                photos.iter().map(|photo| photo.slug.clone()),
            );

    let content_api = Router::new()
        .merge(content_collection_router(
            "/api/writing",
            "/api/writing/{slug}",
            CachedCollection::new_with_not_found(writing, "Post not found"),
        ))
        .merge(content_collection_router(
            "/api/projects",
            "/api/projects/{slug}",
            CachedCollection::new_with_not_found(projects, "Project not found"),
        ))
        .merge(content_collection_router(
            "/api/photos",
            "/api/photos/{slug}",
            CachedCollection::new_with_not_found(photos, "Photo post not found"),
        ));

    let analytics_read_api = analytics_read_router(analytics_state.clone());
    let analytics_write_api = analytics_write_router(analytics_state);
    let system_api = system_router();

    let rate_limit_enabled = env_flag("ENABLE_RATE_LIMIT", true);
    let (content_api, analytics_read_api, analytics_write_api, system_api) = if rate_limit_enabled {
        let trust_proxy_headers = env_flag("RATE_LIMIT_TRUST_PROXY_HEADERS", true);
        let content_api = apply_rate_limit(
            content_api,
            "content-read",
            rate_limit_per_second("content-read", 60),
            rate_limit_burst_size("content-read", 180),
            trust_proxy_headers,
            vec![Method::GET],
        );
        let analytics_read_api = apply_rate_limit(
            analytics_read_api,
            "analytics-read",
            rate_limit_per_second("analytics-read", 10),
            rate_limit_burst_size("analytics-read", 30),
            trust_proxy_headers,
            vec![Method::GET],
        );
        let analytics_write_api = apply_rate_limit(
            analytics_write_api,
            "analytics-write",
            rate_limit_per_second("analytics-write", 5),
            rate_limit_burst_size("analytics-write", 20),
            trust_proxy_headers,
            vec![Method::POST],
        );
        let system_api = apply_rate_limit(
            system_api,
            "system",
            rate_limit_per_second("system", 2),
            rate_limit_burst_size("system", 6),
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

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<HeaderValue>()
                .expect("valid development origin"),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .merge(content_api)
        .merge(analytics_read_api)
        .merge(analytics_write_api)
        .merge(system_api)
        .nest_service("/assets", ServeDir::new("./content"))
        .layer(DefaultBodyLimit::max(1024))
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
