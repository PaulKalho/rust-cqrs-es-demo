use crate::domain::project::aggregate::Project;
use async_trait::async_trait;
use cqrs_es::{persist::GenericQuery, EventEnvelope, Query};
use postgres_es::PostgresViewRepository;
use sqlx::{Pool, Postgres};

use super::{aggregate, view::ProjectView};

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

pub struct AllProjectsQuery {
    pool: Pool<Postgres>,
    table_name: String,
}

impl AllProjectsQuery {
    pub fn new(pool: Pool<Postgres>, table_name: String) -> Self {
        Self { pool, table_name }
    }

    pub async fn collect_all(&self) -> Result<Vec<ProjectView>, Box<dyn std::error::Error>> {
        let select_all_sql = format!("SELECT payload FROM {}", self.table_name);

        // TODO: implement the AllProjectsQuery

        Ok(vec![])
    }
}

#[async_trait]
impl Query<Project> for AllProjectsQuery {
    async fn dispatch(&self, _aggregate_id: &str, _events: &[EventEnvelope<Project>]) {
        let projects = self.collect_all().await.unwrap_or_else(|_| vec![]);
        println!("{:?}", projects);
    }
}
