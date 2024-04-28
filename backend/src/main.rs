mod config;
mod machines;
mod router;
mod utils;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::Config;
use dotenv::dotenv;
use router::create_router;
use serde::Serialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{sync::Arc, time::Duration};
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    env: Config,
}

#[derive(Debug, Serialize)]
pub enum ResponseType {
    Fail,
    Success,
}

#[derive(Debug, Serialize)]
pub struct ResponseData {
    pub status: ResponseType,
    pub message: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let (_file_guard, _terminal_guard) = utils::tracing::init(&config.log_path);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Can't connect to Database");

    let state = AppState {
        db: pool.clone(),
        env: config.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin("127.0.0.1".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(state))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("Can't start listener");

    info!("Listening on 0.0.0.0:80");

    axum::serve(listener, app)
        .await
        .expect("Can't start server");
}