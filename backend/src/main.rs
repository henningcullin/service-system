mod auth;
mod channels;
mod config;
mod machines;
mod reports;
mod router;
mod tasks;
mod users;
mod utils;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::Config;
use dotenv::dotenv;
use router::create_router;
use sqlx::postgres::{PgListener, PgPool, PgPoolOptions};
use std::{sync::Arc, time::Duration};
use tokio::sync::{broadcast::Sender, Mutex};
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Clone)]
pub struct Channels {
    tasks: Arc<Mutex<Sender<String>>>,
    reports: Arc<Mutex<Sender<String>>>,
}

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    env: Config,
    channels: Channels,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let (_file_guard, _terminal_guard) = utils::tracing::init(&config.log_path);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Can't connect to Database");

    let (task_sender, _) = tokio::sync::broadcast::channel(100);
    let (report_sender, _) = tokio::sync::broadcast::channel(100);

    let mut task_listener = PgListener::connect_with(&pool)
        .await
        .expect("Can't connect to Database");
    task_listener
        .listen("task_changed")
        .await
        .expect("Can't listen on task_changed channel");

    let mut report_listener = PgListener::connect_with(&pool)
        .await
        .expect("Can't connect to Database");
    report_listener
        .listen("report_changed")
        .await
        .expect("Can't listen on report_changed channel");

    let cloned_task_sender = task_sender.clone();
    let cloned_report_sender = report_sender.clone();

    tokio::spawn(async move {
        while let Ok(notification) = task_listener.recv().await {
            let _ = cloned_task_sender.send(notification.payload().to_string());
        }
    });

    tokio::spawn(async move {
        while let Ok(notification) = report_listener.recv().await {
            let _ = cloned_report_sender.send(notification.payload().to_string());
        }
    });

    let state = AppState {
        db: pool.clone(),
        env: config.clone(),
        channels: Channels {
            tasks: Arc::new(Mutex::new(task_sender)),
            reports: Arc::new(Mutex::new(report_sender)),
        },
    };

    let cors = CorsLayer::new()
        .allow_origin("127.0.0.1".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(state)).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("Can't start listener");

    info!("Listening on 0.0.0.0:80");

    axum::serve(listener, app)
        .await
        .expect("Can't start server");
}
