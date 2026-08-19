use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("{0}")]
  BadRequest(#[from] BadRequest),
  #[error("JSON deserialization error: {0}")]
  SerdeJsonError(#[from] serde_json::Error),
  #[error("{0}")]
  NotFound(#[from] NotFound),
  #[error("{0}")]
  Conflict(#[from] Conflict),
  #[error("IO error: {0}")]
  StdIoError(#[from] std::io::Error),
  #[error("Internal error: {0}")]
  Internal(String),
}

#[derive(Error, Debug)]
#[error("Bad request: {reason}")]
pub struct BadRequest {
  pub reason: String,
}

#[derive(Error, Debug)]
#[error("Resource not found: {reason}")]
pub struct NotFound {
  pub reason: String,
}

#[derive(Error, Debug)]
#[error("Conflict: {reason}")]
pub struct Conflict {
  pub reason: String,
}

impl Error {
  pub fn bad_request(reason: impl Into<String>) -> Self {
    Self::BadRequest(BadRequest {
      reason: reason.into(),
    })
  }

  pub fn not_found(reason: impl Into<String>) -> Self {
    Self::NotFound(NotFound {
      reason: reason.into(),
    })
  }

  pub fn conflict(reason: impl Into<String>) -> Self {
    Self::Conflict(Conflict {
      reason: reason.into(),
    })
  }

  pub fn internal(reason: impl Into<String>) -> Self {
    Self::Internal(reason.into())
  }
}
