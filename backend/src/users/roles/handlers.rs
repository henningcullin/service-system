use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};

use sqlx::{query, query_as, Postgres, QueryBuilder};

use crate::{
    field_vec, insert_fields, update_field,
    users::models::User,
    utils::{
        check_permission,
        db::{Field, IntoField},
        errors::{ApiError, ForbiddenReason, InputInvalidReason},
    },
    AppState,
};

use super::models::{NewRole, QueryRole, Role, UpdateRole};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryRole>,
) -> Result<Json<Role>, ApiError> {
    check_permission(user.role.user_view)?;

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
    .await?;

    Ok(Json(role))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Role>>, ApiError> {
    check_permission(user.role.user_view)?;

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
    .await?;

    Ok(Json(roles))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewRole>,
) -> Result<(StatusCode, Json<Role>), ApiError> {
    check_permission(user.role.user_create)?;

    if body.level <= user.role.level {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission))?;
    }

    let mut query_builder = QueryBuilder::<Postgres>::new("INSERT INTO roles ( ");

    let fields = field_vec![
        name => body.name,
        level => body.level,
        has_password => body.has_password,
        user_view => body.user_view,
        user_create => body.user_create,
        user_edit => body.user_edit,
        user_delete => body.user_delete,
        machine_view => body.machine_view,
        machine_create => body.machine_create,
        machine_edit => body.machine_edit,
        machine_delete => body.machine_delete,
        task_view => body.task_view,
        task_create => body.task_create,
        task_edit => body.task_edit,
        task_delete => body.task_delete,
        report_view => body.report_view,
        report_create => body.report_create,
        report_edit => body.report_edit,
        report_delete => body.report_delete,
        facility_view => body.facility_view,
        facility_create => body.facility_create,
        facility_edit => body.facility_edit,
        facility_delete => body.facility_delete
    ];

    insert_fields!(query_builder, &fields);

    query_builder.push(" RETURNING *");

    let role = query_builder
        .build_query_as::<Role>()
        .fetch_one(&app_state.db)
        .await?;

    Ok((StatusCode::CREATED, Json(role)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateRole>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.user_edit)?;

    let target_role = query_as!(
        Role,
        r#"
        SELECT
            *
        FROM
            roles r
        WHERE
            r.id = $1 
        "#,
        body.id
    )
    .fetch_one(&app_state.db)
    .await?;

    if target_role.level <= user.role.level {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

    if let Some(level) = body.level {
        if level <= user.role.level {
            return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
        }
    }

    let mut query_builder = QueryBuilder::<Postgres>::new("UPDATE roles SET");
    let mut separated_list = query_builder.separated(",");

    let fields = field_vec![
        name => body.name,
        level => body.level,
        has_password => body.has_password,
        user_view => body.user_view,
        user_create => body.user_create,
        user_edit => body.user_edit,
        user_delete => body.user_delete,
        machine_view => body.machine_view,
        machine_create => body.machine_create,
        machine_edit => body.machine_edit,
        machine_delete => body.machine_delete,
        task_view => body.task_view,
        task_create => body.task_create,
        task_edit => body.task_edit,
        task_delete => body.task_delete,
        report_view => body.report_view,
        report_create => body.report_create,
        report_edit => body.report_edit,
        report_delete => body.report_delete,
        facility_view => body.facility_view,
        facility_create => body.facility_create,
        facility_edit => body.facility_edit,
        facility_delete => body.facility_delete
    ];

    if fields.len() < 1 {
        return Err(ApiError::InputInvalid(InputInvalidReason::NoFieldsToUpdate));
    }

    for (field, value) in fields {
        update_field!(separated_list, field, value);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(body.id);

    let result = query_builder.build().execute(&app_state.db).await?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryRole>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.user_delete)?;

    let target_role = query_as!(
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
    .await?;

    if target_role.level <= user.role.level {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

    let result = query!(r#"DELETE FROM roles WHERE id = $1"#, params.id)
        .execute(&app_state.db)
        .await?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
