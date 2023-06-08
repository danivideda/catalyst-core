use crate::state::State;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
};
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};

mod health;
mod v1;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot run service, error: {0}")]
    CannotRunService(String),
    #[error(transparent)]
    EventDbError(#[from] event_db::error::Error),
}

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    error: String,
}

pub fn app(state: Arc<State>) -> Router {
    // build our application with a route
    let v1 = v1::v1(state);
    let health = health::health();
    Router::new().nest("/api", v1).merge(health)
}

pub async fn run_service(addr: &SocketAddr, state: Arc<State>) -> Result<(), Error> {
    tracing::info!("Starting service...");
    tracing::info!("Listening on {}", addr);

    let app = app(state);

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| Error::CannotRunService(e.to_string()))?;
    Ok(())
}

async fn handle_result<T: Serialize>(res: Result<T, Error>) -> Response {
    match res {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(Error::EventDbError(event_db::error::Error::NotFound(error))) => {
            (StatusCode::NOT_FOUND, Json(ErrorMessage { error })).into_response()
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                error: error.to_string(),
            }),
        )
            .into_response(),
    }
}
