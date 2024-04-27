use async_graphql::http::ALL_WEBSOCKET_PROTOCOLS;
use async_graphql::Data;
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::Response;

use auth::parse_auth_header;
use domain::auth::credentials::Credential;
use serde_json::Value;

use crate::gql::RustySchema;

fn extract_auth_header(headers: &HeaderMap) -> Credential {
    let Some(value) = headers.get("Authorization") else {
        return Credential::None;
    };
    auth::parse_auth_header(value.to_str().unwrap_or(""))
}

fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}

fn parse_query(query: &str) -> (String, String, String) {
    let query = query.splitn(2, ' ').collect::<Vec<&str>>();
    let r#type = query[0];
    let query = query[1];
    let query = query.replace(' ', "");
    let query = remove_whitespace(&query);
    let query = query.splitn(4, '{').skip(1).take(2).collect::<Vec<&str>>();
    (
        r#type.to_string(),
        query[0].to_string(),
        query[1].to_string(),
    )
}

pub async fn graphql_handler(
    State(schema): State<RustySchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let query = req.0.query.clone();
    let req = req
        .into_inner()
        .data(parse_query(&query))
        .data(extract_auth_header(&headers));
    schema.execute(req).await.into()
}

pub async fn on_connection_init(value: Value) -> async_graphql::Result<Data> {
    value.as_object().map_or_else(
        || Err("Invalid payload".into()),
        |payload| {
            payload["auth"].as_str().map_or_else(
                || Err("Auth data missing from payload".into()),
                |auth| {
                    let mut data = Data::default();
                    data.insert(parse_auth_header(auth));
                    Ok(data)
                },
            )
        },
    )
}

pub async fn graphql_ws_handler(
    State(schema): State<RustySchema>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> Response {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema.clone(), protocol)
                .on_connection_init(on_connection_init)
                .serve()
        })
}
