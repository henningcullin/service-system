use axum::{ // Framework
    routing::{ // HTTP Methods
        get,
        post,
        put,
        delete
    }, Router // The Router
};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/car", get(cars::details))
        .route("/cars", get(cars::index))
        .route("/car", post(cars::create))
        .route("/car", put(cars::update))
        .route("/car", delete(cars::delete));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}