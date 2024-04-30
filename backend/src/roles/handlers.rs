use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sqlx::{Postgres, QueryBuilder};

use crate::{utils::errors::ApiError, AppState};

use super::models::{NewRole, QueryRole, Role, UpdateRole};

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryRole>,
) -> Result<Json<Role>, ApiError> {
    let role = sqlx::query_as!(
        Role,
        r#"
        SELECT
            *
        FROM
            roles r
        WHERE
            r.id = $1 
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(role))
}

pub async fn index(State(app_state): State<Arc<AppState>>) -> Result<Json<Vec<Role>>, ApiError> {
    let roles = sqlx::query_as!(
        Role,
        r#"
        SELECT 
            * 
        FROM 
            roles
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(roles))
}

pub async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewRole>,
) -> Result<(StatusCode, Json<Role>), ApiError> {
    let role = sqlx::query_as!(
        Role,
        r#"
        INSERT INTO 
            roles 
        (
            name, 
            level, 
            has_password, 
            user_view, 
            user_create, 
            user_edit, 
            user_delete, 
            machine_view, 
            machine_create, 
            machine_edit, 
            machine_delete, 
            task_view, 
            task_create, 
            task_edit, 
            task_delete, 
            report_view, 
            report_create, 
            report_edit, 
            report_delete, 
            facility_view, 
            facility_create, 
            facility_edit, 
            facility_delete
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
            $8, 
            $9, 
            $10, 
            $11, 
            $12, 
            $13, 
            $14, 
            $15, 
            $16, 
            $17, 
            $18, 
            $19, 
            $20, 
            $21, 
            $22, 
            $23
        )
        RETURNING
            *
        "#,
        body.name,
        body.level,
        body.has_password.unwrap_or(true),
        body.user_view.unwrap_or(false),
        body.user_create.unwrap_or(false),
        body.user_edit.unwrap_or(false),
        body.user_delete.unwrap_or(false),
        body.machine_view.unwrap_or(false),
        body.machine_create.unwrap_or(false),
        body.machine_edit.unwrap_or(false),
        body.machine_delete.unwrap_or(false),
        body.task_view.unwrap_or(false),
        body.task_create.unwrap_or(false),
        body.task_edit.unwrap_or(false),
        body.task_delete.unwrap_or(false),
        body.report_view.unwrap_or(false),
        body.report_create.unwrap_or(false),
        body.report_edit.unwrap_or(false),
        body.report_delete.unwrap_or(false),
        body.facility_view.unwrap_or(false),
        body.facility_create.unwrap_or(false),
        body.facility_edit.unwrap_or(false),
        body.facility_delete.unwrap_or(false)
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(role)))
}

macro_rules! update_field {
    ($query:expr, $field:expr, $value:expr) => {
        if let Some(val) = $value {
            $query.push(concat!(" ", $field, " = ")).push_bind(val);
        }
    };
}

pub async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateRole>,
) -> Result<StatusCode, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new("UPDATE roles SET");

    update_field!(query, "name", body.name);
    update_field!(query, "level", body.level);
    update_field!(query, "has_password", body.has_password);
    update_field!(query, "user_view", body.user_view);
    update_field!(query, "user_create", body.user_create);
    update_field!(query, "user_edit", body.user_edit);
    update_field!(query, "user_delete", body.user_delete);
    update_field!(query, "machine_view", body.machine_view);
    update_field!(query, "machine_create", body.machine_create);
    update_field!(query, "machine_edit", body.machine_edit);
    update_field!(query, "machine_delete", body.machine_delete);
    update_field!(query, "task_view", body.task_view);
    update_field!(query, "task_create", body.task_create);
    update_field!(query, "task_edit", body.task_edit);
    update_field!(query, "task_delete", body.task_delete);
    update_field!(query, "report_view", body.report_view);
    update_field!(query, "report_create", body.report_create);
    update_field!(query, "report_edit", body.report_edit);
    update_field!(query, "report_delete", body.report_delete);
    update_field!(query, "facility_view", body.facility_view);
    update_field!(query, "facility_create", body.facility_create);
    update_field!(query, "facility_edit", body.facility_edit);
    update_field!(query, "facility_delete", body.facility_delete);

    query.push(" WHERE id = ");
    query.push_bind(body.id);

    let result = query.build()
        .execute(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => {
            Ok(StatusCode::OK)
        }
        _ => {
            Ok(StatusCode::NOT_FOUND)
        }
    }
}
