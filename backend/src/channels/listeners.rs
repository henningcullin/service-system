use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    Extension,
};
use futures_util::stream::{self, Stream};
use std::{convert::Infallible, sync::Arc};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use crate::{
    users::models::User,
    utils::{check_permission, errors::ApiError},
    AppState,
}; // import your AppState

pub async fn task_listen(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    check_permission(user.role.task_view)?;

    let reciever = app_state.channels.tasks.lock().await.subscribe();

    let stream = BroadcastStream::new(reciever);

    let sse_stream = stream::unfold(stream, |mut stream| async {
        match stream.next().await {
            Some(Ok(data)) => {
                let event = Event::default().data(data);
                Some((Ok(event), stream))
            }
            _ => None,
        }
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}

pub async fn report_listen(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    check_permission(user.role.report_view)?;

    let reciever = app_state.channels.reports.lock().await.subscribe();

    let stream = BroadcastStream::new(reciever);

    let sse_stream = stream::unfold(stream, |mut stream| async {
        match stream.next().await {
            Some(Ok(data)) => {
                let event = Event::default().data(data);
                Some((Ok(event), stream))
            }
            _ => None,
        }
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}
