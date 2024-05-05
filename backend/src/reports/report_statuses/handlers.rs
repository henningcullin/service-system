use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::{query, query_as};

use crate::{
    users::models::User,
    utils::{check_permission, errors::ApiError},
    AppState,
};

use super::{
    models::{NewReportStatus, QueryReportStatus, UpdateReportStatus},
    ReportStatus,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryReportStatus>,
) -> Result<Json<ReportStatus>, ApiError> {
    check_permission(user.role.report_view)?;

    let report_status = query_as!(
        ReportStatus,
        r#"
        SELECT
            *
        FROM
            report_statuses rs
        WHERE
            rs.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(report_status))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<ReportStatus>>, ApiError> {
    check_permission(user.role.report_view)?;

    let report_statuses = query_as!(
        ReportStatus,
        r#"
        SELECT
            *
        FROM
        report_statuses
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(report_statuses))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewReportStatus>,
) -> Result<(StatusCode, Json<ReportStatus>), ApiError> {
    check_permission(user.role.report_create)?;

    let report_status = query_as!(
        ReportStatus,
        r#"
        INSERT INTO
            report_statuses
        (
            name
        )
        VALUES
        (
            $1
        )
        RETURNING
            *
        "#,
        body.name
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(report_status)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateReportStatus>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.report_edit)?;

    let result = query!(
        r#"
        UPDATE 
            report_statuses rs
        SET
            name = $1
        WHERE
            rs.id = $2
        "#,
        body.name,
        body.id
    )
    .execute(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryReportStatus>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.report_delete)?;

    let result = query!(r#"DELETE FROM report_statuses WHERE id = $1"#, params.id)
        .execute(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
