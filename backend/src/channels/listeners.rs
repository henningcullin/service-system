use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    Extension,
};
use futures_util::stream::{self, Stream};
use sqlx::postgres::PgListener;
use std::{convert::Infallible, sync::Arc};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use crate::{utils::errors::ApiError, AppState}; // import your AppState

pub async fn task_listen(
    State(app_state): State<Arc<AppState>>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    let mut listener = PgListener::connect_with(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    listener
        .listen("task_changed")
        .await
        .map_err(ApiError::from)?;

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Ok(notification) = listener.recv().await {
            let _ = tx.send(notification.payload().to_string()).await;
        }
    });

    let stream = ReceiverStream::new(rx);

    let sse_stream = stream::unfold(stream, |mut stream| async {
        match stream.next().await {
            Some(data) => {
                let event = Event::default().data(data);
                Some((Ok(event), stream))
            }
            None => None,
        }
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}

pub async fn report_listen(
    State(app_state): State<Arc<AppState>>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    let mut listener = PgListener::connect_with(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    listener
        .listen("report_changed")
        .await
        .map_err(ApiError::from)?;

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Ok(notification) = listener.recv().await {
            let _ = tx.send(notification.payload().to_string()).await;
        }
    });

    let stream = ReceiverStream::new(rx);

    let sse_stream = stream::unfold(stream, |mut stream| async {
        match stream.next().await {
            Some(data) => {
                let event = Event::default().data(data);
                Some((Ok(event), stream))
            }
            None => None,
        }
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}
