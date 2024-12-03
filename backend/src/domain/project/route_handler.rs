use crate::config::database::ApplicationState;
use crate::domain::project::commands::ProjectCommand;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use cqrs_es::persist::ViewRepository;

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

pub async fn command_handler(
    Path(project_id): Path<String>,
    State(state): State<ApplicationState>,
) -> Response {
    match state
        .cqrs
        .execute(
            &project_id,
            ProjectCommand::CreateProject {
                project_id: (project_id.to_string()),
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
