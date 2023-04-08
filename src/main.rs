use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

use crate::config::Config;

mod commands;
mod config;
mod framework;

#[derive(Debug, clap::Parser)]
struct CliArgs {
    #[arg(short, long, default_value = "config.toml")]
    config_path: PathBuf,

    #[arg(long, default_value = "false")]
    overwrite_config: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();
    let config_path = args.config_path;

    if !config_path.exists() || args.overwrite_config {
        if !config_path.exists() {
            eprintln!("Config file does not exist, creating one with default values");
        }
        let config = Config::default();
        config.save(&config_path).await?;
        return Ok(());
    }

    let config = Config::load(&config_path)
        .await
        .map_err(|e| anyhow!("Failed to load the config: {e}"))?;

    let fw = framework::new(config);

    fw.run().await.map_err(anyhow::Error::msg)
}
