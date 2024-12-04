use crate::domain::project::{aggregate::Project, events::ProjectEvent};
use chrono::{DateTime, Utc};
use cqrs_es::View;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectView {
    project_id: uuid::Uuid,
    project_name: String,
    project_start: DateTime<Utc>,
    project_end: DateTime<Utc>,
    participants_name: Vec<String>,
}

impl View<Project> for ProjectView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Project>) {
        match &event.payload {
            ProjectEvent::ProjectCreated {
                project_id,
                project_name,
                project_start,
                project_end,
                participants_name,
            } => {
                self.project_id = project_id.clone();
                self.project_name = project_name.to_string();
                self.project_start = *project_start;
                self.project_end = *project_end;
                self.participants_name = participants_name.to_vec();
            }
        }
    }
}

pub type ProjectViewRepository = PostgresViewRepository<ProjectView, Project>;
