use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::domain::project::{
    commands::ProjectCommand,
    events::{ProjectError, ProjectEvent},
    services::ProjectServices,
};

#[derive(Serialize, Deserialize)]
pub struct Project {
    project_id: String,
}

#[async_trait]
impl Aggregate for Project {
    type Command = ProjectCommand;
    type Event = ProjectEvent;
    type Error = ProjectError;
    type Services = ProjectServices;

    fn aggregate_type() -> String {
        "Project".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            ProjectCommand::CreateProject { project_id } => {
                Ok(vec![ProjectEvent::ProjectCreated {
                    project_id: project_id,
                }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ProjectEvent::ProjectCreated { project_id } => self.project_id = project_id,
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Project {
            project_id: "".to_string(),
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use std::sync::Mutex;

    use crate::domain::project::services::{CreateError, ProjectApi};

    use super::*;
    use cqrs_es::test::TestFramework;

    type ProjectTestFramework = TestFramework<Project>;

    #[test]
    fn test_create() {
        let expected = ProjectEvent::ProjectCreated {
            project_id: "1".to_string(),
        };
        let services = ProjectServices::new(Box::new(MockProjectServices::default()));

        ProjectTestFramework::with(services)
            .given_no_previous_events()
            .when(ProjectCommand::CreateProject {
                project_id: "1".to_string(),
            })
            .then_expect_events(vec![expected]);
    }

    pub struct MockProjectServices {
        create_project: Mutex<Option<Result<(), CreateError>>>,
    }

    impl Default for MockProjectServices {
        fn default() -> Self {
            Self {
                create_project: Mutex::new(None),
            }
        }
    }

    impl MockProjectServices {
        fn set_create_project(&self, response: Result<(), CreateError>) {
            *self.create_project.lock().unwrap() = Some(response)
        }
    }

    #[async_trait]
    impl ProjectApi for MockProjectServices {
        async fn create_project(&self, project_id: &str) -> Result<(), CreateError> {
            self.create_project.lock().unwrap().take().unwrap()
        }
    }
}
