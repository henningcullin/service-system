use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::query;

use crate::{
    machines::models::ShortMachine,
    tasks::models::Task,
    users::models::{ShortUser, User},
    utils::{
        check_permission,
        errors::{ApiError, ForbiddenReason},
    },
    AppState,
};

use super::{
    models::{NewTask, QueryTask},
    task_documents::TaskDocument,
    task_statuses::TaskStatus,
    task_types::TaskType,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryTask>,
) -> Result<Json<Vec<Task>>, ApiError> {
    let user_id = user.id;

    let permissions_ok = user.role.task_view
        || params.creator_id.map_or(false, |id| id == user_id)
        || params.executor_id.map_or(false, |id| id == user_id);

    if !permissions_ok {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

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
                FROM 
                    task_executors te
                INNER JOIN 
                    users u 
                ON 
                    te.user_id = u.id
                WHERE 
                    te.task_id = t.id 
            ) AS "executors: Vec<ShortUser>",
            (
                SELECT array_agg(
                    (
                        td.uri,
                        td.name,
                        td.description
                    )
                )
                FROM 
                    task_documents td
                WHERE 
                    td.task_id = t.id
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
        WHERE
            ($1::UUID IS NULL OR t.id = $1)
        AND
            ($2::UUID IS NULL OR t.creator = $2)
        AND
            ($3::UUID IS NULL OR t.id IN (
                SELECT te.task_id FROM task_executors te WHERE te.user_id = $3
            ))
        "#,
        params.task_id,
        params.creator_id,
        params.executor_id
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(tasks))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Task>>, ApiError> {
    check_permission(user.role.task_view)?;

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

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    check_permission(user.role.task_create)?;

    let mut tx = app_state.db.begin().await.map_err(ApiError::from)?;

    let task_id = sqlx::query_scalar!(
        r#"
        INSERT INTO
            tasks
        (
            title,
            description,
            task_type,
            status,
            archived,
            creator,
            machine,
            due_at
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8
        )
        RETURNING
            id
        "#,
        body.title,
        body.description,
        body.task_type,
        body.status,
        body.archived.unwrap_or(false),
        user.id,
        body.machine,
        body.due_at
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(ApiError::from)?;

    if let Some(executors) = body.executors {
        query!(
            r#"
            INSERT INTO
                task_executors
            (
                task_id,
                user_id
            )
            SELECT $1, unnest($2::uuid[])
            "#,
            task_id,
            &executors
        )
        .execute(&mut *tx)
        .await
        .map_err(ApiError::from)?;
    }

    let task = sqlx::query_as!(
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
                FROM 
                    task_executors te
                INNER JOIN 
                    users u 
                ON 
                    te.user_id = u.id
                WHERE 
                    te.task_id = t.id 
            ) AS "executors: Vec<ShortUser>",
            (
                SELECT array_agg(
                    (
                        td.uri,
                        td.name,
                        td.description
                    )
                )
                FROM 
                    task_documents td
                WHERE 
                    td.task_id = t.id
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
        WHERE
            t.id = $1
        "#,
        task_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(ApiError::from)?;

    tx.commit().await.map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(task)))
}
