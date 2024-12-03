use axum::{routing::get, Router};
use ip_demo::config::database::configure_state;
use ip_demo::domain::project::route_handler::{command_handler, query_handler};

#[tokio::main]
async fn main() {
    let state = configure_state().await;

    let router = Router::new()
        .route(
            "/project/:project_id",
            get(query_handler).post(command_handler),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
