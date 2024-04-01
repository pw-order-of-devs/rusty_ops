use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

/// CORS settings
pub fn cors_layer() -> CorsLayer {
    CorsLayer::default()
        .allow_methods([Method::OPTIONS, Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any)
}
