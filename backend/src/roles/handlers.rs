use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

enum Field {
    Str(Option<String>),
    Int(Option<i32>),
    Bool(Option<bool>),
    // Add more variants here as needed
}

use sqlx::{query, query_as, Postgres, QueryBuilder};

use crate::{update_field, utils::errors::ApiError, AppState};

use super::models::{NewRole, QueryRole, Role, UpdateRole};

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryRole>,
) -> Result<Json<Role>, ApiError> {
    let role = query_as!(
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
    let roles = query_as!(
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
    let role = query_as!(
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

pub async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateRole>,
) -> Result<StatusCode, ApiError> {
    let mut query_builder = QueryBuilder::<Postgres>::new("UPDATE roles SET");
    let mut separated_list = query_builder.separated(",");

    let fields = vec![
        ("name", Field::Str(body.name)),
        ("level", Field::Int(body.level)),
        ("has_password", Field::Bool(body.has_password)),
        ("user_view", Field::Bool(body.user_view)),
        ("user_create", Field::Bool(body.user_create)),
        ("user_edit", Field::Bool(body.user_edit)),
        ("user_delete", Field::Bool(body.user_delete)),
        ("machine_view", Field::Bool(body.machine_view)),
        ("machine_create", Field::Bool(body.machine_create)),
        ("machine_edit", Field::Bool(body.machine_edit)),
        ("machine_delete", Field::Bool(body.machine_delete)),
        ("task_view", Field::Bool(body.task_view)),
        ("task_create", Field::Bool(body.task_create)),
        ("task_edit", Field::Bool(body.task_edit)),
        ("task_delete", Field::Bool(body.task_delete)),
        ("report_view", Field::Bool(body.report_view)),
        ("report_create", Field::Bool(body.report_create)),
        ("report_edit", Field::Bool(body.report_edit)),
        ("report_delete", Field::Bool(body.report_delete)),
        ("facility_view", Field::Bool(body.facility_view)),
        ("facility_create", Field::Bool(body.facility_create)),
        ("facility_edit", Field::Bool(body.facility_edit)),
        ("facility_delete", Field::Bool(body.facility_delete)),
    ];

    for (field, value) in fields {
        update_field!(separated_list, field, value);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(body.id);

    let result = query_builder.build()
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

pub async fn delete(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryRole>,
) -> Result<StatusCode, ApiError> {
    let result = query!(
        r#"DELETE FROM roles WHERE id = $1"#,
        params.id
    )
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