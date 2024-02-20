mod machine;
mod user;
mod task;
mod auth;
mod config;
mod router;

use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};
use router::create_router;
use tower_http::cors::CorsLayer;
use std::{
    sync::Arc,
    time::Duration
};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

use config::Config;


#[derive(Clone)]
pub struct AppState {
    db: Pool<MySql>,
    env: Config
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(7)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Can't connect to Database");


    let state = AppState {
        db: pool.clone(),
        env: config.clone()
    };
    
    let cors = CorsLayer::new()
        .allow_origin("127.0.0.1".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(state))
        .layer(cors);
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.expect("Can't start listener");
    axum::serve(listener, app).await.expect("Can't start server");
}