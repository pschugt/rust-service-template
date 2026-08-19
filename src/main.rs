use std::net::SocketAddr;
use std::sync::LazyLock;

use {{crate_name}}::api::router::serve;
use {{crate_name}}::config::settings::SETTINGS;
use {{crate_name}}::errors::error::Error;
use tracing_subscriber::{Layer, Registry, layer::SubscriberExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
  LazyLock::force(&SETTINGS);

  let stdout_filter = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| (&SETTINGS.default_logging.as_str()).into());
  let stdout_layer = tracing_subscriber::fmt::layer()
    .with_ansi(true)
    .with_file(true)
    .with_line_number(true)
    .with_filter(stdout_filter);

  let logger = Registry::default().with(stdout_layer);
  tracing::subscriber::set_global_default(logger).unwrap();

  println!("Starting server on port {}...", SETTINGS.port);
  let address = SocketAddr::from(([0, 0, 0, 0], SETTINGS.port));

  serve(address).await;
  Ok(())
}
