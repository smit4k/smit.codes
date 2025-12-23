use super::views::{get_view_count, has_recent_view, record_view, ViewCount};
use crate::AppState;
use axum::{
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::net::{IpAddr, SocketAddr};

/// Extracts the IP address from headers or socket
fn extract_ip(headers: &HeaderMap, addr: &SocketAddr) -> IpAddr {
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(ip_str) = forwarded_str.split(',').next() {
                if let Ok(ip) = ip_str.trim().parse() {
                    return ip;
                }
            }
        }
    }

    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse() {
                return ip;
            }
        }
    }

    addr.ip()
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

    if post_type != "posts" && post_type != "projects" {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if the slug exists in AppState
    let post_slug = match post_type.as_str() {
        "posts" => state
            .writing
            .iter()
            .find(|p| p.slug == slug)
            .map(|p| p.slug.clone()),
        "projects" => state
            .projects
            .iter()
            .find(|p| p.slug == slug)
            .map(|p| p.slug.clone()),
        _ => None,
    }
    .ok_or(StatusCode::NOT_FOUND)?;

    let viewer_ip = extract_ip(&headers, &addr);
    let ip_string = viewer_ip.to_string();

    // Check if already viewed in last 24 hours
    match has_recent_view(pool, &post_slug, &ip_string).await {
        Ok(true) => return Ok(StatusCode::NO_CONTENT),
        Ok(false) => {}
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    // Get user agent
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let type_singular = post_type.trim_end_matches('s');

    match record_view(pool, &post_slug, type_singular, Some(ip_string), user_agent).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
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
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
