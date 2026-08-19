use async_trait::async_trait;

use crate::{
  errors::error::Error, models::example::Example,
  repositories::example_repository::ExampleRepository,
};

pub struct ExampleService;

impl ExampleService {
  pub fn new() -> Self {
    Self
  }
}

impl Default for ExampleService {
  fn default() -> Self {
    Self::new()
  }
}

#[async_trait]
impl ExampleRepository for ExampleService {
  async fn get_by_id(&self, id: u64) -> Result<Example, Error> {
    if id == 0 {
      return Err(Error::not_found("example with id 0"));
    }

    Ok(Example {
      id,
      name: format!("Example {id}"),
    })
  }
}
