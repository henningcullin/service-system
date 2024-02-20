mod machine;
mod user;
mod task;
mod auth;
mod model;
mod response;
mod config;

use axum::{ // Framework
    routing::{ // HTTP Methods
        get,
        post,
        put,
        delete
    }, 
    Router, // The Router
};
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


    let api = Router::new()

        .route("/machine", get(machine::details))
        .route("/machines", get(machine::index))
        .route("/machine", post(machine::create))
        .route("/machine", delete(machine::delete))
        .route("/machine", put(machine::update));
    
    let app = Router::new()
    
        .nest("/api", api)
        
        .with_state(Arc::new(state));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.expect("Can't start listener");
    axum::serve(listener, app).await.expect("Can't start server");
}