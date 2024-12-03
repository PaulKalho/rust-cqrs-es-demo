use crate::domain::project::aggregate::Project;
use async_trait::async_trait;
use cqrs_es::{persist::GenericQuery, EventEnvelope, Query};
use postgres_es::PostgresViewRepository;

use super::view::ProjectView;

pub struct SimpleLoggingQuery {}

#[async_trait]
impl Query<Project> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Project>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload)
        }
    }
}

pub type ProjectQuery =
    GenericQuery<PostgresViewRepository<ProjectView, Project>, ProjectView, Project>;
