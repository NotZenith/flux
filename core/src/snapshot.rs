use anyhow::{Result, Context};
use std::path::Path;
use std::fs;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotMetadata {
    pub id: String,
    pub message: String,
    pub timestamp: i64,
    pub files_path: String,
    pub docker_volumes: Vec<String>,
}

pub struct SnapshotManager {
    base_path: String,
}

impl SnapshotManager {
    pub fn new(base_path: &str) -> Self {
        let path = format!("{}/.flux/snapshots", base_path);
        fs::create_dir_all(&path).expect("Failed to create snapshots directory");
        Self { base_path: path }
    }

    pub fn create_snapshot(&self, message: &str, target_dir: &str, volumes: Vec<String>) -> Result<String> {
        let id = format!("snap_{}", Utc::now().timestamp());
        let snap_path = format!("{}/{}", self.base_path, id);
        fs::create_dir_all(&snap_path)?;

        // 1. Snapshot local files (excluding node_modules, .git, etc.)
        self.archive_files(target_dir, &format!("{}/files.tar.gz", snap_path))?;

        // 2. Snapshot Docker volumes (if any)
        for volume in &volumes {
            let _ = self.snapshot_docker_volume(volume, &format!("{}/vol_{}.tar", snap_path, volume));
        }

        let metadata = SnapshotMetadata {
            id: id.clone(),
            message: message.to_string(),
            timestamp: Utc::now().timestamp_millis(),
            files_path: format!("{}/files.tar.gz", snap_path),
            docker_volumes: volumes,
        };

        fs::write(
            format!("{}/metadata.json", snap_path),
            serde_json::to_string_pretty(&metadata)?
        )?;

        Ok(id)
    }

    fn snapshot_docker_volume(&self, volume: &str, destination: &str) -> Result<()> {
        // Run a temporary container to export the volume content
        let status = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-v")
            .arg(format!("{}:/data", volume))
            .arg("-v")
            .arg(format!("{}:/backup", Path::new(destination).parent().unwrap().to_str().unwrap()))
            .arg("alpine")
            .arg("tar")
            .arg("-cvf")
            .arg(format!("/backup/{}", Path::new(destination).file_name().unwrap().to_str().unwrap()))
            .arg("-C")
            .arg("/data")
            .arg(".")
            .status()
            .context("Failed to export docker volume")?;

        if !status.success() {
            anyhow::bail!("Docker volume snapshot failed");
        }
        Ok(())
    }

    fn archive_files(&self, source: &str, destination: &str) -> Result<()> {
        // Use system 'tar' command for high-performance archiving
        let status = Command::new("tar")
            .arg("-czf")
            .arg(destination)
            .arg("--exclude=.git")
            .arg("--exclude=node_modules")
            .arg("-C")
            .arg(source)
            .arg(".")
            .status()
            .context("Failed to execute tar command")?;

        if !status.success() {
            anyhow::bail!("Tar command failed with status: {}", status);
        }

        Ok(())
    }

    pub fn list_snapshots(&self) -> Result<Vec<SnapshotMetadata>> {
        let mut results = vec![];
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let meta_path = path.join("metadata.json");
                if meta_path.exists() {
                    let content = fs::read_to_string(meta_path)?;
                    let meta: SnapshotMetadata = serde_json::from_str(&content)?;
                    results.push(meta);
                }
            }
        }
        Ok(results)
    }
}
