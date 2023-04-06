mod display;
mod player;

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Serialize, Debug)]
enum ApiError {}

#[derive(Clone)]
struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() {
    #[cfg(degbug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[cfg(not(degbug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = axum::Router::new().with_state(AppState::new());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
