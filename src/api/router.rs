use axum::{Json, Router};
use http::{HeaderName, StatusCode};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tracing::info;

use crate::api::routes;
use crate::handlers::example_handler::ExampleHandler;
use crate::models::status::Status;
use crate::services::example_service::ExampleService;

#[derive(Clone)]
pub struct AppState {
  pub example_handler: Arc<ExampleHandler>,
}

const DEFAULT_SENSITIVE_HEADERS: [HeaderName; 4] = [
  http::header::AUTHORIZATION,
  http::header::PROXY_AUTHORIZATION,
  http::header::COOKIE,
  http::header::SET_COOKIE,
];

/// The app function that configures and creates the Axum router for our API
pub fn app(state: AppState) -> Router {
  Router::new()
    .nest(
      "/v1",
      Router::new()
        .merge(routes::status::create_route())
        .merge(routes::example::create_route()),
    )
    .fallback(fallback)
    .layer(SetSensitiveHeadersLayer::from_shared(Arc::new(
      DEFAULT_SENSITIVE_HEADERS,
    )))
    .layer(CorsLayer::permissive())
    .with_state(state)
}
/// The serve function that starts the Axum server
pub async fn serve(address: SocketAddr) {
  let listener = tokio::net::TcpListener::bind(address).await.unwrap();
  let example_repository = Arc::new(ExampleService::new());
  let example_handler = Arc::new(ExampleHandler::new(example_repository));

  let state = AppState { example_handler };

  info!("Server listening on {}", &address);
  axum::serve(listener, app(state).into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Failed to start server");
}

/// The fallback function when a non configured endpoint is reached
async fn fallback() -> (StatusCode, Json<Status>) {
  (
    StatusCode::NOT_FOUND,
    Json(Status {
      status: "Not found".to_owned(),
    }),
  )
}

/// Setup example of the OS signal handlers to catch for proper server graceful shutdown.
async fn shutdown_signal() {
  let ctrl_c = async {
    tokio::signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
    _ = ctrl_c => {},
    _ = terminate => {},
  }
}
