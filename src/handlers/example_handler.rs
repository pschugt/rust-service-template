use std::sync::Arc;

use crate::{
  errors::error::Error, models::example::Example,
  repositories::example_repository::ExampleRepository,
};

pub struct ExampleHandler {
  repository: Arc<dyn ExampleRepository>,
}

impl ExampleHandler {
  pub fn new(repository: Arc<dyn ExampleRepository>) -> Self {
    Self { repository }
  }

  pub async fn get_example(&self, id: u64) -> Result<Example, Error> {
    self.repository.get_by_id(id).await
  }
}
