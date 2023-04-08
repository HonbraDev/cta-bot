use anyhow::Result;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub discor_token: String,
    pub cool_people: HashSet<serenity::UserId>,
}

impl Config {
    pub async fn load(path: &PathBuf) -> Result<Self> {
        let file = tokio::fs::read_to_string(path).await?;
        let config = toml::from_str(&file)?;
        Ok(config)
    }

    pub async fn save(&self, path: &PathBuf) -> Result<()> {
        let file = toml::to_string(&self)?;
        tokio::fs::write(path, file).await?;
        Ok(())
    }
}
