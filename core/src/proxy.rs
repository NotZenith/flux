use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use anyhow::{Result, Context};
use serde::Serialize;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize)]
pub struct TrafficCapture {
    pub id: Uuid,
    pub timestamp: i64,
    pub method: String,
    pub path: String,
    pub host: String,
    pub status_code: Option<u16>,
    pub duration_ms: Option<u64>,
}

pub struct FluxProxy {
    port: u16,
    events_tx: tokio::sync::broadcast::Sender<TrafficCapture>,
}

impl FluxProxy {
    pub fn new(port: u16) -> Self {
        let (tx, _) = tokio::sync::broadcast::channel(1000);
        Self {
            port,
            events_tx: tx,
        }
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<TrafficCapture> {
        self.events_tx.subscribe()
    }

    pub async fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        println!("FluxProxy listening on port {}", self.port);

        loop {
            let (socket, _) = listener.accept().await?;
            let tx = self.events_tx.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket, tx).await {
                    eprintln!("Proxy connection error: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(
    mut client_stream: TcpStream,
    tx: tokio::sync::broadcast::Sender<TrafficCapture>,
) -> Result<()> {
    let mut buffer = [0u8; 4096];
    let n = client_stream.read(&mut buffer).await?;
    if n == 0 { return Ok(()); }

    let request_head = String::from_utf8_lossy(&buffer[..n]);

    let lines: Vec<&str> = request_head.lines().collect();
    if lines.is_empty() { return Ok(()); }

    let first_line: Vec<&str> = lines[0].split_whitespace().collect();
    if first_line.len() < 2 { return Ok(()); }

    let method = first_line[0].to_string();
    let path = first_line[1].to_string();

    let mut host = String::new();
    for line in lines.iter().skip(1) {
        if line.to_lowercase().starts_with("host:") {
            host = line[5..].trim().to_string();
            break;
        }
    }

    let start_time = Utc::now();
    let capture_id = Uuid::new_v4();

    let _ = tx.send(TrafficCapture {
        id: capture_id,
        timestamp: start_time.timestamp_millis(),
        method,
        path,
        host: host.clone(),
        status_code: None,
        duration_ms: None,
    });

    let target_addr = if host.contains("auth") { "127.0.0.1:8081" } else { "127.0.0.1:8080" };

    let mut server_stream = TcpStream::connect(target_addr).await
        .context(format!("Failed to connect to target service at {}", target_addr))?;

    server_stream.write_all(&buffer[..n]).await?;

    let (mut client_read, mut client_write) = client_stream.split();
    let (mut server_read, mut server_write) = server_stream.split();

    let _ = tokio::try_join!(io::copy(&mut client_read, &mut server_write), io::copy(&mut server_read, &mut client_write));

    Ok(())
}
