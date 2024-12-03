use std::env;
use std::sync::Arc;

use crate::domain::project::{aggregate::Project, view::ProjectView};
use postgres_es::{default_postgress_pool, PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres};

use crate::config::cqrs::configure_cqrs;

#[derive(Clone)]
pub struct ApplicationState {
    pub cqrs: Arc<PostgresCqrs<Project>>,
    pub project_query: Arc<PostgresViewRepository<ProjectView, Project>>,
}

pub async fn configure_state() -> ApplicationState {
    let connection_string = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/postgres".to_string());
    let pool: Pool<Postgres> = default_postgress_pool(&connection_string).await;
    let (cqrs, project_query) = configure_cqrs(pool);
    ApplicationState {
        cqrs,
        project_query,
    }
}
