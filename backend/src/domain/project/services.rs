use async_trait::async_trait;

pub struct ProjectServices {
    pub services: Box<dyn ProjectApi>,
}

impl ProjectServices {
    pub fn new(services: Box<dyn ProjectApi>) -> Self {
        Self { services }
    }
}

#[async_trait]
pub trait ProjectApi: Sync + Send {
    async fn create_project(&self, project_id: &str) -> Result<(), CreateError>;
}
pub struct CreateError;

pub struct HappyPathProjectServices;

#[async_trait]
impl ProjectApi for HappyPathProjectServices {
    async fn create_project(&self, project_id: &str) -> Result<(), CreateError> {
        Ok(())
    }
}
