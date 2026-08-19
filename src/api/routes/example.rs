use crate::{api::router::AppState, errors::error::Error, models::example::Example};
use axum::{
  Json, Router,
  extract::{Path, State},
  routing::get,
};

pub fn create_route() -> Router<AppState> {
  Router::new().route("/examples/{id}", get(get_example))
}

async fn get_example(
  State(state): State<AppState>,
  Path(id): Path<u64>,
) -> Result<Json<Example>, Error> {
  let example = state.example_handler.get_example(id).await?;

  Ok(Json(example))
}
