use config::{Config, Environment};
use dotenvy::dotenv;
use serde::Deserialize;
use std::sync::LazyLock;

pub static SETTINGS: LazyLock<Settings> =
  LazyLock::new(|| Settings::new().expect("failed to load configuration"));

/// The Settings struct where the required config for our app is loaded into
#[derive(Deserialize, Debug)]
pub struct Settings {
  /// The server port to listen to.
  #[serde(default = "default_app_port")]
  pub port: u16,
  /// We make use of tracing with rust logging integration (RUST_LOG).
  #[serde(default = "default_logging")]
  pub default_logging: String,
}

fn default_app_port() -> u16 {
  8080
}
fn default_logging() -> String {
  "{{crate_name}}=debug,tower_http=trace,axum::rejection=trace".to_owned()
}

impl Settings {
  pub fn new() -> Result<Self, config::ConfigError> {
    dotenv().ok();
    let config = Config::builder()
      .add_source(Environment::default())
      .build()?;
    config.try_deserialize()
  }
}
