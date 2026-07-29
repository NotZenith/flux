use clap::{Parser, Subcommand};
use anyhow::Result;
use flux_core::{ServiceManager, FluxConfig, FluxProxy, bridge::FluxBridge};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("{} Initializing Flux project...", "🚀".green());
            let default_config = FluxConfig {
                project_name: "my-flux-project".to_string(),
                services: vec![],
                network: Some(flux_core::config::NetworkConfig {
                    proxy_port: Some(8888),
                    enable_interception: true,
                }),
            };
            default_config.save_to_file("flux.json")?;
            println!("Created flux.json");
            Ok(())
        }
        Commands::Start => {
            println!("{} Starting Flux stack...", "⚡".green());

            let config = FluxConfig::load_from_file("flux.json")?;
            let service_manager = Arc::new(ServiceManager::new());
            let proxy = Arc::new(FluxProxy::new(config.network.as_ref().and_then(|n| n.proxy_port).unwrap_or(8888)));

            let bridge = FluxBridge::new(
                9999,
                service_manager.subscribe(),
                service_manager.log_engine().subscribe(),
                proxy.subscribe(),
            );

            // Start everything
            let sm = service_manager.clone();
            tokio::spawn(async move {
                let _ = bridge.start().await;
            });

            let px = proxy.clone();
            tokio::spawn(async move {
                let _ = px.start().await;
            });

            for s_cfg in config.services {
                let id = sm.add_service(s_cfg.name, s_cfg.command, s_cfg.args.unwrap_or_default());
                sm.start_service(id)?;
            }

            println!("Flux is running. Open the Desktop UI to observe.");

            // Keep main alive
            tokio::signal::ctrl_c().await?;
            println!("Shutting down...");
            Ok(())
        }
        Commands::Stop => {
            println!("{} Stopping all services...", "🛑".yellow());
            Ok(())
        }
        Commands::Logs { service } => {
            if let Some(s) = service {
                println!("Streaming logs for {}...", s);
            } else {
                println!("Streaming all logs...");
            }
            Ok(())
        }
        Commands::Snap { message } => {
            println!("{} Creating snapshot: {}", "📸".blue(), message);
            Ok(())
        }
        Commands::Restore { snapshot_id } => {
            println!("{} Restoring snapshot {}...", "🔄".magenta(), snapshot_id);
            Ok(())
        }
    }
}
