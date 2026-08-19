use crate::errors::error::Error;
use axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;
use tracing::{debug, info};

#[derive(Error, Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ApiError {
  #[error("Bad request")]
  BadRequest = 40001,
  #[error("Invalid JSON")]
  SerdeJsonError = 40002,
  #[error("Resource not found")]
  NotFound = 40401,
  #[error("Conflict")]
  Conflict = 40901,
  #[error("Internal server error")]
  InternalError = 50001,
}

impl ApiError {
  pub fn status(self) -> StatusCode {
    match self {
      ApiError::BadRequest => StatusCode::BAD_REQUEST,
      ApiError::SerdeJsonError => StatusCode::UNPROCESSABLE_ENTITY,
      ApiError::NotFound => StatusCode::NOT_FOUND,
      ApiError::Conflict => StatusCode::CONFLICT,
      ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

/// JSON response structure for API errors.
#[derive(Serialize, Deserialize)]
pub struct ApiErrorResponse {
  pub code: ApiError,
  pub message: String,
}

impl From<&Error> for ApiError {
  fn from(error: &Error) -> Self {
    match error {
      Error::BadRequest(_) => ApiError::BadRequest,
      Error::SerdeJsonError(_) => ApiError::SerdeJsonError,
      Error::NotFound(_) => ApiError::NotFound,
      Error::Conflict(_) => ApiError::Conflict,
      Error::StdIoError(_) | Error::Internal(_) => ApiError::InternalError,
    }
  }
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let code: ApiError = (&self).into();
    let status = code.status();
    let message = code.to_string();

    debug!(
      error_code = code as u16,
      status = status.as_u16(),
      internal_error = %self,
      "Returning API error"
    );

    info!(
      counter.api_error = 1,
      status_code = status.as_u16(),
      error_code = code as u16,
      "api_error"
    );

    (status, Json(ApiErrorResponse { code, message })).into_response()
  }
}
