use std::time::Duration;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
  #[serde(deserialize_with = "from_millis")]
  pub read_timeout: Duration,
  #[serde(deserialize_with = "from_millis")]
  pub restart_delay: Duration,
  #[serde(deserialize_with = "from_millis")]
  pub scan_retry_delay: Duration,
  #[serde(deserialize_with = "from_millis")]
  pub scan_timeout: Duration,
  pub monitor: MonitorConfig,
  pub rpc: RpcConfig,
  pub osc: OscConfig,
  pub log: LogConfig,
  pub file: FileConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MonitorConfig {
  pub freeze_last_value: bool,
  #[serde(deserialize_with = "from_millis_optional")]
  pub freeze_timeout: Option<Duration>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RpcConfig {
  pub enable: bool,
  pub id: String,
  #[serde(deserialize_with = "from_millis")]
  pub update_interval: Duration,
  pub templates: RpcTemplates,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RpcTemplates {
  pub details: String,
  pub state: String,
  pub frozen_details: String,
  pub frozen_state: String,
  pub disconnected_details: String,
  pub disconnected_state: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OscConfig {
  pub enable: bool,
  pub host: String,
  pub port: u16,
  #[serde(deserialize_with = "from_millis")]
  pub update_interval: Duration,
  pub percent_min: u8,
  pub percent_max: u8,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LogConfig {
  pub enable: bool,
  pub write_zero: bool,
  #[serde(deserialize_with = "from_millis")]
  pub update_interval: Duration,
  pub path: String,
  pub templates: LogTemplates,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LogTemplates {
  pub template: String,
  pub frozen_template: String,
  pub disconnected_template: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FileConfig {
  pub enable: bool,
  #[serde(deserialize_with = "from_millis")]
  pub update_interval: Duration,
  pub template: String,
  pub path: String,
}

fn from_millis<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where D: serde::Deserializer<'de> {
  Ok(Duration::from_millis(Deserialize::deserialize(deserializer)?))
}

fn from_millis_optional<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where D: serde::Deserializer<'de> {
  let millis = Deserialize::deserialize(deserializer)?;

  if millis == 0 {
    Ok(None)
  } else {
    Ok(Some(Duration::from_millis(millis)))
  }
}

const CONFIG_PATH: &str = "config.toml";

const DEFAULT_CONFIG: &str = include_str!("../../config.example.toml");

pub fn load_config() -> anyhow::Result<Config> {
  let data = match std::fs::read_to_string(CONFIG_PATH) {
    Ok(data) => data,
    Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
      // write default config
      std::fs::write(CONFIG_PATH, DEFAULT_CONFIG)?;
      warn!("config.toml not found, created with default values");
      DEFAULT_CONFIG.to_string()
    }
    Err(err) => return Err(err.into()),
  };

  Ok(toml::from_str(&data)?)
}
