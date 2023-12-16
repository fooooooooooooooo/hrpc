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
  pub rpc: RpcConfig,
  pub osc: OscConfig,
  pub log: LogConfig,
  pub file: FileConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RpcConfig {
  pub enable: bool,
  #[serde(deserialize_with = "from_string")]
  pub id: u64,
  #[serde(deserialize_with = "from_millis")]
  pub update_interval: Duration,
  pub templates: RpcTemplates,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RpcTemplates {
  pub details: String,
  pub state: String,
  pub na_details: String,
  pub na_state: String,
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
  pub template: String,
  pub path: String,
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

fn from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where D: serde::Deserializer<'de> {
  deserializer.deserialize_any(StringOrNumberVisitor)
}

struct StringOrNumberVisitor;

impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
  type Value = u64;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("string or number")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where E: serde::de::Error {
    value.parse().map_err(serde::de::Error::custom)
  }

  fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
  where E: serde::de::Error {
    Ok(value)
  }
}

const CONFIG_PATH: &str = "config.toml";

pub fn load_config() -> anyhow::Result<Config> {
  let data = std::fs::read_to_string(CONFIG_PATH)?;

  Ok(toml::from_str(&data)?)
}
