use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use sqlx::query_as;

use crate::{machines::facilities::Facility, utils::errors::ApiError, AppState};

use super::{
    models::{QueryUser, User},
    roles::models::Role,
};

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryUser>,
) -> Result<Json<User>, ApiError> {
    let user = query_as!(
        User,
        r#"
        SELECT
            u.id,
            u.first_name,
            u.last_name,
            u.email,
            u.phone,
            (
                r.id,
                r.name,
                r.level,
                r.has_password,
                r.user_view,
                r.user_create,
                r.user_edit,
                r.user_delete,
                r.machine_view,
                r.machine_create,
                r.machine_edit,
                r.machine_delete,
                r.task_view,
                r.task_create,
                r.task_edit,
                r.task_delete,
                r.report_view,
                r.report_create,
                r.report_edit,
                r.report_delete,
                r.facility_view,
                r.facility_create,
                r.facility_edit,
                r.facility_delete
            ) AS "role!: Role",
            u.active,
            u.last_login,
            u.occupation,
            u.image,
            (
                f.id,
                f.name,
                f.address
            ) AS "facility?: Facility"
        FROM
            users u
        INNER JOIN
            roles r
        ON
            u.role = r.id
        LEFT JOIN
            facilities f
        ON
            u.facility = f.id
        WHERE
            u.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(user))
}

pub async fn index(State(app_state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, ApiError> {
    let users = query_as!(
        User,
        r#"
        SELECT
            u.id,
            u.first_name,
            u.last_name,
            u.email,
            u.phone,
            (
                r.id,
                r.name,
                r.level,
                r.has_password,
                r.user_view,
                r.user_create,
                r.user_edit,
                r.user_delete,
                r.machine_view,
                r.machine_create,
                r.machine_edit,
                r.machine_delete,
                r.task_view,
                r.task_create,
                r.task_edit,
                r.task_delete,
                r.report_view,
                r.report_create,
                r.report_edit,
                r.report_delete,
                r.facility_view,
                r.facility_create,
                r.facility_edit,
                r.facility_delete
            ) AS "role!: Role",
            u.active,
            u.last_login,
            u.occupation,
            u.image,
            (
                f.id,
                f.name,
                f.address
            ) AS "facility?: Facility"
        FROM
            users u
        INNER JOIN
            roles r
        ON
            u.role = r.id
        LEFT JOIN
            facilities f
        ON
            u.facility = f.id
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(users))
}
