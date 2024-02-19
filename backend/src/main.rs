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

    let database_url = env::var("DATABASE_URL").expect("Can't resolve DATABASE_URL");

    let pool = MySqlPoolOptions::new()
        .max_connections(7)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Can't connect to Database");


    let api = Router::new()

    .route("/machine", get(machine::details))
    .route("/machines", get(machine::index))
    .route("/machine", post(machine::create))
    .route("/machine", delete(machine::delete))
    .route("/machine", put(machine::update));
    
    let app = Router::new()
    
        .nest("/api", api)
        .with_state(pool);
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.expect("Can't start listener");
    axum::serve(listener, app).await.expect("Can't start server");
}