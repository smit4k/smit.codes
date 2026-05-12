#[cfg(feature = "rate-limit")]
use ::axum::http::Method;
use ::axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
#[cfg(feature = "rate-limit")]
use tower_governor::{
    governor::GovernorConfigBuilder,
    key_extractor::{PeerIpKeyExtractor, SmartIpKeyExtractor},
    GovernorLayer,
};
#[cfg(feature = "rate-limit")]
use tracing::info;

#[cfg(feature = "system")]
use crate::system::{
    models::{SystemInfo, SystemMetrics},
    monitor::{get_system_info, get_system_metrics},
};
use crate::{
    content::models::{ContentItem, PhotoPost},
    utils::cache,
};

pub trait Slugged {
    fn slug(&self) -> &str;
}

impl Slugged for ContentItem {
    fn slug(&self) -> &str {
        &self.slug
    }
}

impl Slugged for PhotoPost {
    fn slug(&self) -> &str {
        &self.slug
    }
}

#[derive(Clone)]
pub struct CachedPayload {
    pub etag: HeaderValue,
    pub body: Bytes,
}

#[derive(Clone)]
pub struct CachedCollection<T> {
    pub items: Arc<Vec<T>>,
    pub list: Arc<CachedPayload>,
    pub by_slug: Arc<HashMap<String, CachedPayload>>,
    not_found_message: &'static str,
}

impl<T> CachedCollection<T>
where
    T: Serialize + Slugged,
{
    pub fn new(items: Vec<T>) -> Self {
        Self::new_with_not_found(items, "Content not found")
    }

    pub fn new_with_not_found(items: Vec<T>, not_found_message: &'static str) -> Self {
        let list = Arc::new(cached_payload(&items));
        let by_slug = Arc::new(
            items
                .iter()
                .map(|item| (item.slug().to_string(), cached_payload(item)))
                .collect(),
        );

        Self {
            items: Arc::new(items),
            list,
            by_slug,
            not_found_message,
        }
    }
}

pub fn cached_payload<T: Serialize>(value: &T) -> CachedPayload {
    let body = Bytes::from(serde_json::to_vec(value).expect("Failed to serialize cached payload"));
    let etag = cache::etag_from_bytes(&body);

    CachedPayload { etag, body }
}

pub fn respond_cached(headers: &HeaderMap, payload: &CachedPayload) -> Response {
    if let Some(if_none_match) = headers.get("If-None-Match") {
        if if_none_match == payload.etag {
            return StatusCode::NOT_MODIFIED.into_response();
        }
    }

    let response_headers = cache::default_cache_headers(payload.etag.clone());
    (response_headers, payload.body.clone()).into_response()
}

pub fn content_collection_router<T>(
    list_route: &'static str,
    detail_route: &'static str,
    collection: CachedCollection<T>,
) -> Router
where
    T: Serialize + Slugged + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(list_route, get(list_collection::<T>))
        .route(detail_route, get(get_collection_item::<T>))
        .with_state(collection)
}

async fn list_collection<T>(
    State(collection): State<CachedCollection<T>>,
    headers: HeaderMap,
) -> Response
where
    T: Serialize + Slugged + Clone + Send + Sync + 'static,
{
    respond_cached(&headers, collection.list.as_ref())
}

async fn get_collection_item<T>(
    State(collection): State<CachedCollection<T>>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Response, (StatusCode, &'static str)>
where
    T: Serialize + Slugged + Clone + Send + Sync + 'static,
{
    let payload = collection
        .by_slug
        .get(&slug)
        .ok_or((StatusCode::NOT_FOUND, collection.not_found_message))?;
    Ok(respond_cached(&headers, payload))
}

#[cfg(feature = "system")]
pub fn system_router() -> Router {
    Router::new()
        .route("/api/system/info", get(system_info_handler))
        .route("/api/system/metrics", get(system_metrics_handler))
}

#[cfg(feature = "system")]
async fn system_info_handler() -> Json<SystemInfo> {
    Json(get_system_info())
}

#[cfg(feature = "system")]
async fn system_metrics_handler() -> Json<SystemMetrics> {
    Json(get_system_metrics())
}

#[cfg(feature = "rate-limit")]
pub fn apply_rate_limit<S>(
    router: Router<S>,
    _policy_name: &str,
    per_second: u64,
    burst_size: u32,
    trust_proxy_headers: bool,
    methods: Vec<Method>,
) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    if trust_proxy_headers {
        let mut builder = GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .use_headers();
        builder
            .per_second(per_second)
            .burst_size(burst_size)
            .methods(methods);
        router.layer(GovernorLayer::new(Arc::new(
            builder.finish().expect("invalid rate limit config"),
        )))
    } else {
        let mut builder = GovernorConfigBuilder::default()
            .key_extractor(PeerIpKeyExtractor)
            .use_headers();
        builder
            .per_second(per_second)
            .burst_size(burst_size)
            .methods(methods);
        router.layer(GovernorLayer::new(Arc::new(
            builder.finish().expect("invalid rate limit config"),
        )))
    }
}

#[cfg(feature = "rate-limit")]
pub fn log_rate_limit_policy(policy_name: &str, per_second: u64, burst_size: u32) {
    info!(
        "Rate limit policy {}: per_second={}, burst_size={}",
        policy_name, per_second, burst_size
    );
}
