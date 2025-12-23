use axum::http::{HeaderMap, HeaderValue};
use sha1::{Digest, Sha1};

pub fn etag_from_bytes(bytes: &[u8]) -> HeaderValue {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let hash = format!("\"{:x}\"", hasher.finalize());
    HeaderValue::from_str(&hash).unwrap()
}

pub fn default_cache_headers(etag: HeaderValue) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert("ETag", etag);
    headers.insert(
        "Cache-Control",
        HeaderValue::from_static("public, max-age=0, must-revalidate"),
    );

    headers
}
