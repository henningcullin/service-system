use sqlx::{postgres::PgListener, Pool, Postgres};
use tokio::sync::broadcast::Sender;

pub async fn init_channels(pool: &Pool<Postgres>) -> (Sender<String>, Sender<String>) {
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

    (task_sender, report_sender)
}
