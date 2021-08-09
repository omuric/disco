use anyhow::{Context, Result};
use serde_derive::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Config {
    pub webhook_url: String,
}

pub async fn read_config() -> Result<Config> {
    let home_dir = dirs::home_dir().with_context(|| format!("Not found home directory."))?;
    let config_path = &format!("{}/.config/disco/config.json", home_dir.display());
    let json = fs::read_to_string(config_path).await.with_context(|| {
        format!(
            "Not found configuration file. (config_path={})",
            config_path
        )
    })?;
    let config = serde_json::from_str(&json)
        .with_context(|| format!("Invalid config file. (config_path={})", config_path))?;
    Ok(config)
}
