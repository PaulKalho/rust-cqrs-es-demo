use async_trait::async_trait;
use chrono::prelude::*;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::domain::project::{
    commands::ProjectCommand,
    events::{ProjectError, ProjectEvent},
    services::ProjectServices,
};

#[derive(Serialize, Deserialize)]
pub struct Project {
    project_id: uuid::Uuid,
    project_name: String,
    project_start: DateTime<Utc>,
    project_end: DateTime<Utc>,
    participants_name: Vec<String>,
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
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            ProjectCommand::CreateProject {
                project_id,
                project_name,
                project_start,
                project_end,
                participants_name,
            } => Ok(vec![ProjectEvent::ProjectCreated {
                project_id: project_id,
                project_name: project_name,
                project_start: project_start,
                project_end: project_end,
                participants_name: participants_name,
            }]),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ProjectEvent::ProjectCreated {
                project_id,
                project_name,
                project_start,
                project_end,
                participants_name,
            } => {
                self.project_id = project_id;
                self.project_name = project_name;
                self.project_start = project_start;
                self.project_end = project_end;
                self.participants_name = participants_name
            }
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Project {
            project_id: uuid::Uuid::new_v4(),
            project_name: "".to_string(),
            project_start: Utc::now(),
            project_end: Utc::now(),
            participants_name: vec![],
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
        let uuid = uuid::Uuid::new_v4();
        let project_start = Utc::now();
        let project_end = Utc::now();

        let expected = ProjectEvent::ProjectCreated {
            project_id: uuid,
            project_name: "Test Name".to_string(),
            project_start,
            project_end,
            participants_name: vec!["test participant".to_string(), "second".to_string()],
        };
        let services = ProjectServices::new(Box::new(MockProjectServices::default()));

        ProjectTestFramework::with(services)
            .given_no_previous_events()
            .when(ProjectCommand::CreateProject {
                project_id: uuid,
                project_name: "Test Name".to_string(),
                project_start,
                project_end,
                participants_name: vec!["test participant".to_string(), "second".to_string()],
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
        async fn create_project(&self, _project_id: &uuid::Uuid) -> Result<(), CreateError> {
            self.create_project.lock().unwrap().take().unwrap()
        }
    }
}
