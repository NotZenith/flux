use serde::{Serialize, Deserialize};
use tokio::process::{Child, Command};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use anyhow::{Result, Context};
use tokio::sync::broadcast;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Starting,
    Running,
    Stopped,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub status: ServiceStatus,
    pub created_at: DateTime<Utc>,
    pub pid: Option<u32>,
}

use crate::logger::LogEngine;

pub struct ServiceManager {
    services: Arc<Mutex<HashMap<Uuid, Service>>>,
    processes: Arc<Mutex<HashMap<Uuid, Child>>>,
    events_tx: broadcast::Sender<ServiceEvent>,
    log_engine: Arc<LogEngine>,
}

#[derive(Debug, Clone, Serialize)]
pub enum ServiceEvent {
    StatusChanged { id: Uuid, status: ServiceStatus },
    LogReceived { id: Uuid, content: String },
}

impl ServiceManager {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            services: Arc::new(Mutex::new(HashMap::new())),
            processes: Arc::new(Mutex::new(HashMap::new())),
            events_tx: tx,
            log_engine: Arc::new(LogEngine::new()),
        }
    }

    pub fn add_service(&self, name: String, command: String, args: Vec<String>) -> Uuid {
        let id = Uuid::new_v4();
        let service = Service {
            id,
            name,
            command,
            args,
            env: HashMap::new(),
            status: ServiceStatus::Stopped,
            created_at: Utc::now(),
            pid: None,
        };
        self.services.lock().unwrap().insert(id, service);
        id
    }

    pub fn start_service(&self, id: Uuid) -> Result<()> {
        let mut services = self.services.lock().unwrap();
        let service = services.get_mut(&id).context("Service not found")?;

        if let ServiceStatus::Running = service.status {
            return Ok(());
        }

        service.status = ServiceStatus::Starting;
        self.broadcast_event(ServiceEvent::StatusChanged { id, status: ServiceStatus::Starting });

        let mut child = Command::new(&service.command)
            .args(&service.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(format!("Failed to start service: {}", service.name))?;

        service.pid = Some(child.id());
        service.status = ServiceStatus::Running;

        let log_engine = self.log_engine.clone();
        tokio::spawn(async move {
            let _ = log_engine.capture_child(id, child).await;
        });

        self.broadcast_event(ServiceEvent::StatusChanged { id, status: ServiceStatus::Running });

        Ok(())
    }

    pub fn stop_service(&self, id: Uuid) -> Result<()> {
        let mut processes = self.processes.lock().unwrap();
        if let Some(mut child) = processes.remove(&id) {
            child.kill()?;
            let mut services = self.services.lock().unwrap();
            if let Some(service) = services.get_mut(&id) {
                service.status = ServiceStatus::Stopped;
                service.pid = None;
                self.broadcast_event(ServiceEvent::StatusChanged { id, status: ServiceStatus::Stopped });
            }
        }
        Ok(())
    }

    pub fn get_services(&self) -> Vec<Service> {
        self.services.lock().unwrap().values().cloned().collect()
    }

    fn broadcast_event(&self, event: ServiceEvent) {
        let _ = self.events_tx.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ServiceEvent> {
        self.events_tx.subscribe()
    }

    pub fn log_engine(&self) -> Arc<LogEngine> {
        self.log_engine.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_manager_init() {
        let manager = ServiceManager::new();
        assert_eq!(manager.get_services().len(), 0);
    }

    #[tokio::test]
    async fn test_add_service() {
        let manager = ServiceManager::new();
        let id = manager.add_service("test".to_string(), "ls".to_string(), vec![]);
        assert_eq!(manager.get_services().len(), 1);
        assert_eq!(manager.get_services()[0].name, "test");
    }
}
