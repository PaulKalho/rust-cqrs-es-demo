use axum::http::{HeaderName, Method};
use axum::{routing::get, Router};
use ip_demo::config::database::configure_state;
use ip_demo::domain::project::route_handler::{command_handler, query_handler};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let state = configure_state().await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ]);

    let router = Router::new()
        .route("/project/", get(query_handler).post(command_handler))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
