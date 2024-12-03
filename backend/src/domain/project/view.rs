use crate::domain::project::{aggregate::Project, events::ProjectEvent};
use cqrs_es::View;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectView {
    project_id: Option<String>,
}

impl View<Project> for ProjectView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Project>) {
        match &event.payload {
            ProjectEvent::ProjectCreated { project_id } => {
                self.project_id = Some(project_id.clone())
            }
        }
    }
}

pub type ProjectViewRepository = PostgresViewRepository<ProjectView, Project>;
