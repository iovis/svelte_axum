use axum::{routing::get, Json, Router};
use backend::app_tracing;
use backend::report_error::ReportError;
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    app_tracing::setup();

    let app = Router::new()
        .route("/", get(hello))
        .route("/users", get(users))
        .route("/error", get(failing_handler))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn hello() -> &'static str {
    "Hello World!"
}

#[derive(Debug, Serialize)]
struct User {
    id: Uuid,
    username: String,
}

#[axum::debug_handler]
async fn users() -> Json<Vec<User>> {
    let users = get_users().await;
    tracing::debug!("debug from users!");

    Json(users)
}

#[tracing::instrument(ret)]
async fn get_users() -> Vec<User> {
    tracing::info!("info from get_users!");

    vec![
        User {
            id: Uuid::new_v4(),
            username: "iovis".to_owned(),
        },
        User {
            id: Uuid::new_v4(),
            username: "muxi".to_owned(),
        },
    ]
}

#[axum::debug_handler]
#[tracing::instrument(ret, err)]
pub async fn failing_handler() -> Result<String, ReportError> {
    Err(color_eyre::eyre::eyre!("failing_handler is failing!"))?;

    Ok("this message should not happen?".to_string())
}
