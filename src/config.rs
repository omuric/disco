use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use tokio::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub webhook_url: String,
}

fn config_dir() -> Result<String> {
    let home_dir = dirs::home_dir().with_context(|| "Not found home directory.".to_string())?;
    let config_dir = format!("{}/.config/disco", home_dir.display());
    Ok(config_dir)
}

fn config_path() -> Result<String> {
    let config_path = format!("{}/config.json", config_dir()?);
    Ok(config_path)
}

pub async fn read_config() -> Result<Config> {
    let config_path = &config_path()?;
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

pub async fn write_config(config: Config) -> Result<()> {
    let config_path = &config_path()?;
    let config_dir = config_dir()?;
    let config = serde_json::to_string(&config)
        .with_context(|| "Failed serialize for configuration.".to_string())?;
    fs::create_dir_all(config_dir).await?;
    fs::write(config_path, config).await.with_context(|| {
        format!(
            "Failed write configuration file. (config_path={})",
            config_path
        )
    })?;
    Ok(())
}
