use crate::config::database::ApplicationState;
use crate::domain::project::commands::ProjectCommand;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use cqrs_es::persist::ViewRepository;
use serde::Deserialize;

pub async fn query_handler(
    Path(project_id): Path<String>,
    State(state): State<ApplicationState>,
) -> Response {
    let view = match state.project_query.load(&project_id).await {
        Ok(view) => view,
        Err(err) => {
            println!("Error: {:#?}\n", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };
    match view {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(account_view) => (StatusCode::OK, Json(account_view)).into_response(),
    }
}

#[derive(Deserialize)]
pub struct CreateProjectDTO {
    project_name: String,
    project_start: DateTime<Utc>,
    project_end: DateTime<Utc>,
    participants_name: Vec<String>,
}

pub async fn command_handler(
    State(state): State<ApplicationState>,
    Json(payload): Json<CreateProjectDTO>,
) -> Response {
    let newly_created_uuid = uuid::Uuid::new_v4();
    match state
        .cqrs
        .execute(
            &newly_created_uuid.to_string(),
            ProjectCommand::CreateProject {
                project_id: newly_created_uuid,
                project_name: payload.project_name,
                project_start: payload.project_start,
                project_end: payload.project_end,
                participants_name: payload.participants_name,
            },
        )
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            println!("Error {:#?}\n", err);
            (StatusCode::BAD_REQUEST, err.to_string()).into_response()
        }
    }
}
