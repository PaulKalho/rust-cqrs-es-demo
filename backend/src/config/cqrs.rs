use std::sync::Arc;

use crate::domain::project::{
    aggregate::Project,
    queries::{AllProjectsQuery, ProjectQuery},
    services::{HappyPathProjectServices, ProjectServices},
    view::ProjectView,
};
use cqrs_es::Query;
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres};

pub fn configure_cqrs(
    pool: Pool<Postgres>,
) -> (
    Arc<PostgresCqrs<Project>>,
    Arc<PostgresViewRepository<ProjectView, Project>>,
) {
    let project_view_repo = Arc::new(PostgresViewRepository::new("project_view", pool.clone()));
    let mut project_query = ProjectQuery::new(project_view_repo.clone());
    let all_projects_query = AllProjectsQuery::new(pool.clone(), "project_view".to_string());

    project_query.use_error_handler(Box::new(|e| println!("{}", e)));

    let queries: Vec<Box<dyn Query<Project>>> =
        vec![Box::new(project_query), Box::new(all_projects_query)];
    let services = ProjectServices::new(Box::new(HappyPathProjectServices));
    (
        Arc::new(postgres_es::postgres_cqrs(pool, queries, services)),
        project_view_repo,
    )
}
