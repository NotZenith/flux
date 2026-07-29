use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Child;
use tokio::sync::broadcast;
use uuid::Uuid;
use anyhow::Result;
use std::sync::Arc;
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub service_id: Uuid,
    pub timestamp: i64,
    pub content: String,
    pub level: LogLevel,
}

#[derive(Debug, Clone, Serialize)]
pub enum LogLevel {
    Stdout,
    Stderr,
}

pub struct LogEngine {
    tx: broadcast::Sender<LogEntry>,
}

impl LogEngine {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<LogEntry> {
        self.tx.subscribe()
    }

    pub async fn capture_child(&self, service_id: Uuid, mut child: Child) -> Result<()> {
        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        let tx = self.tx.clone();

        // Stdout capture
        let stdout_tx = tx.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let entry = LogEntry {
                    service_id,
                    timestamp: Utc::now().timestamp_millis(),
                    content: line,
                    level: LogLevel::Stdout,
                };
                let _ = stdout_tx.send(entry);
            }
        });

        // Stderr capture
        let stderr_tx = tx.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let entry = LogEntry {
                    service_id,
                    timestamp: Utc::now().timestamp_millis(),
                    content: line,
                    level: LogLevel::Stderr,
                };
                let _ = stderr_tx.send(entry);
            }
        });

        Ok(())
    }
}
