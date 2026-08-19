use crate::{errors::error::Error, models::example::Example};
use async_trait::async_trait;

#[async_trait]
pub trait ExampleRepository: Send + Sync {
  async fn get_by_id(&self, id: u64) -> Result<Example, Error>;
}
