use futures_util::stream::SplitStream;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::http::header;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream};

use commons::env::var_or_default;
use commons::errors::RustyError;

use crate::resolver::assignment::assign_pipeline;

pub(crate) async fn pipeline_created_subscription(uuid: &str) -> Result<(), RustyError> {
    // Initialize subscription read channel
    let mut read = initialize_connection(uuid).await?;

    // Process incoming messages
    while let Some(message) = read.next().await {
        match message? {
            Message::Text(text) => {
                let value = serde_json::from_str::<Value>(&text)?;
                match value["payload"].as_object() {
                    Some(payload) => {
                        if payload["data"].as_object().is_some() {
                            assign_pipeline(uuid, &text).await?;
                        } else if payload["errors"].as_array().is_some() {
                            let errors = payload["errors"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|err| err["message"].as_str().unwrap_or(""))
                                .collect::<Vec<&str>>();
                            log::error!("subscription error occurred: {:?}", errors);
                        } else {
                            continue;
                        };
                    }
                    None => continue,
                }
            }
            other => log::debug!("Unknown message: {other:?}"),
        }
    }

    Ok(())
}

async fn initialize_connection(
    uuid: &str,
) -> Result<SplitStream<tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>>, RustyError>
{
    let host = var_or_default("SERVER_HOST", "localhost".to_string());
    let port = var_or_default("SERVER_PORT", "8000".to_string());
    let request = tokio_tungstenite::tungstenite::http::Request::builder()
        .method("GET")
        .uri(format!("ws://{host}:{port}/ws"))
        .header(header::SEC_WEBSOCKET_PROTOCOL, "graphql-ws")
        .header(header::SEC_WEBSOCKET_KEY, "graphql-ws-key")
        .header(header::SEC_WEBSOCKET_VERSION, "13")
        .header(header::HOST, host)
        .header(header::CONNECTION, "Upgrade")
        .header(header::UPGRADE, "websocket")
        .body(())
        .unwrap();

    let (ws_stream, _) = connect_async(request).await?;
    let (mut write, mut read) = ws_stream.split();
    log::debug!("WebSocket handshake has been successfully completed");

    let credential = format!("Basic {}", crate::api::get_credential()?);
    let subscribe_message = json!({
        "type": "connection_init",
        "payload": { "auth": credential },
    })
    .to_string();
    write.send(Message::Text(subscribe_message)).await?;

    match read.next().await {
        None => {
            log::debug!("Connection error: no ack response");
        }
        Some(res) => match res {
            Ok(resp) => assert_eq!(
                resp,
                Message::Text("{\"type\":\"connection_ack\"}".to_string())
            ),
            Err(err) => return Err(RustyError::AsyncGraphqlError(err.to_string())),
        },
    };

    let subscribe_message = json!({
        "type": "start",
        "id": uuid,
        "payload": { "query": "subscription { pipelines { id number status startDate registerDate jobId agentId } }" },
    })
        .to_string();
    write.send(Message::Text(subscribe_message)).await?;
    Ok(read)
}
