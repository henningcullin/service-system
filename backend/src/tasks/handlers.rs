use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Extension, Json};
use sqlx::query;

use crate::{
    machines::models::ShortMachine,
    tasks::models::Task,
    users::models::{ShortUser, User},
    utils::errors::ApiError,
    AppState,
};

use super::{task_documents::TaskDocument, task_statuses::TaskStatus, task_types::TaskType};

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Task>>, ApiError> {
    let tasks = sqlx::query_as!(
        Task,
        r#"
        SELECT
            t.id,
            t.title,
            t.description,
            (
                tt.id,
                tt.name
            ) AS "task_type!: TaskType",
            (
                ts.id,
                ts.name
            ) AS "status!: TaskStatus",
            t.archived,
            (
                u.id,
                u.first_name,
                u.last_name,
                u.email,
                u.image
            ) AS "creator!: ShortUser",
            (
                SELECT array_agg(
                    (
                        te.user_id,
                        u.first_name,
                        u.last_name,
                        u.email,
                        u.image
                    )
                )
                FROM task_executors te
                INNER JOIN users u ON te.user_id = u.id
                WHERE te.task_id = t.id
            ) AS "executors: Vec<ShortUser>",
            (
                SELECT array_agg(
                    (
                        td.uri,
                        td.name,
                        td.description
                    )
                )
                FROM task_documents td
                WHERE td.task_id = t.id
            ) AS "documents: Vec<TaskDocument>",
            (
                m.id,
                m.name,
                m.make,
                m.image
            ) AS "machine?: ShortMachine",
            t.created,
            t.edited,
            t.due_at
        FROM
            tasks t
        INNER JOIN
            task_types tt 
        ON
            t.task_type = tt.id
        INNER JOIN
            task_statuses ts 
        ON
            t.status = ts.id
        INNER JOIN
            users u
        ON
            t.creator = u.id
        INNER JOIN
            machines m
        ON
            t.machine = m.id
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(tasks))
}
