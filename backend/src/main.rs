use axum::{ // Framework
    routing::{ // HTTP Methods
        get,
        post,
        put,
        delete
    }, Router // The Router
};

use std::env;
use dotenv::dotenv;

mod facility;
mod machine;
mod user;
mod task;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    println!("{}", database_url);

    let app = Router::new()
        .route("/facility", get(facility::details));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}