use chrono::{DateTime, Utc};
use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectEvent {
    ProjectCreated {
        project_id: uuid::Uuid,
        project_name: String,
        project_start: DateTime<Utc>,
        project_end: DateTime<Utc>,
        participants_name: Vec<String>,
    },
}

impl DomainEvent for ProjectEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            ProjectEvent::ProjectCreated { .. } => "ProjectCreated",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug)]
pub struct ProjectError(String);

impl Display for ProjectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ProjectError {}

impl From<&str> for ProjectError {
    fn from(message: &str) -> Self {
        ProjectError(message.to_string())
    }
}
