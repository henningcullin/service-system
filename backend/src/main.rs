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
    env, 
    time::Duration
};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

mod machine;
mod user;
mod task;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(7)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Can't connect to Database");

    let app = Router::new()
        // API

        // MACHINE

        .route("/api/machine", get(machine::details))
        .route("/api/machines", get(machine::index))
        .route("/api/machine", post(machine::create))

        .with_state(pool);
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}