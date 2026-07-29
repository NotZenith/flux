use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::broadcast;
use serde::Serialize;
use anyhow::Result;

use crate::service::ServiceEvent;
use crate::logger::LogEntry;
use crate::proxy::TrafficCapture;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum BridgeEvent {
    Service(ServiceEvent),
    Log(LogEntry),
    Traffic(TrafficCapture),
}

pub struct FluxBridge {
    port: u16,
    service_events: broadcast::Receiver<ServiceEvent>,
    log_events: broadcast::Receiver<LogEntry>,
    traffic_events: broadcast::Receiver<TrafficCapture>,
}

impl FluxBridge {
    pub fn new(
        port: u16,
        service_events: broadcast::Receiver<ServiceEvent>,
        log_events: broadcast::Receiver<LogEntry>,
        traffic_events: broadcast::Receiver<TrafficCapture>,
    ) -> Self {
        Self {
            port,
            service_events,
            log_events,
            traffic_events,
        }
    }

    pub async fn start(self) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        println!("WebSocket bridge listening on ws://127.0.0.1:{}", self.port);

        let service_tx = Arc::new(self.service_events);
        let log_tx = Arc::new(self.log_events);
        let traffic_tx = Arc::new(self.traffic_events);

        while let Ok((stream, _)) = listener.accept().await {
            let s_rx = service_tx.resubscribe();
            let l_rx = log_tx.resubscribe();
            let t_rx = traffic_tx.resubscribe();

            tokio::spawn(async move {
                if let Ok(ws_stream) = accept_async(stream).await {
                    handle_client(ws_stream, s_rx, l_rx, t_rx).await;
                }
            });
        }
        Ok(())
    }
}

async fn handle_client(
    mut ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    mut s_rx: broadcast::Receiver<ServiceEvent>,
    mut l_rx: broadcast::Receiver<LogEntry>,
    mut t_rx: broadcast::Receiver<TrafficCapture>,
) {
    loop {
        tokio::select! {
            Ok(event) = s_rx.recv() => {
                let msg = serde_json::to_string(&BridgeEvent::Service(event)).unwrap();
                if ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.is_err() { break; }
            }
            Ok(event) = l_rx.recv() => {
                let msg = serde_json::to_string(&BridgeEvent::Log(event)).unwrap();
                if ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.is_err() { break; }
            }
            Ok(event) = t_rx.recv() => {
                let msg = serde_json::to_string(&BridgeEvent::Traffic(event)).unwrap();
                if ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.is_err() { break; }
            }
        }
    }
}
