use poem::http::Method;
use poem::middleware::Cors;

/// extract allowed origin
pub fn get_allowed_origin() -> String {
    option_env!("CORS_ALLOW_ORIGIN")
        .unwrap_or("http://localhost:8080")
        .to_string()
}

/// CORS settings
pub fn cors_config() -> Cors {
    Cors::new()
        .allow_methods(vec![Method::POST, Method::OPTIONS])
        .allow_origin(&get_allowed_origin())
        .allow_header("Access-Control-Allow-Origin")
        .allow_header("Content-Type")
        .allow_credentials(true)
}
