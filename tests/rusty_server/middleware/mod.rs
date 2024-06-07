use rusty_server::middleware::cors::cors_layer;

#[test]
fn cors_middleware_test() {
    let layer = cors_layer();
    assert_eq!(
        r#"CorsLayer { allow_credentials: No, allow_headers: Const(Some("*")), allow_methods: Const(Some("OPTIONS,GET,POST")), allow_origin: Const("*"), allow_private_network: No, expose_headers: Const(None), max_age: Exact(None), vary: Vary(["origin", "access-control-request-method", "access-control-request-headers"]) }"#,
        format!("{layer:?}"),
    );
}
