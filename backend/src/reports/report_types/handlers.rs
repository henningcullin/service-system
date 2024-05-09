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
    models::{NewReportType, QueryReportType, UpdateReportType},
    ReportType,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryReportType>,
) -> Result<Json<ReportType>, ApiError> {
    check_permission(user.role.report_view)?;

    let report_type = query_as!(
        ReportType,
        r#"
        SELECT
            *
        FROM
            report_types rt
        WHERE
            rt.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok(Json(report_type))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<ReportType>>, ApiError> {
    check_permission(user.role.report_view)?;

    let report_types = query_as!(
        ReportType,
        r#"
        SELECT
            *
        FROM
            report_types
        "#
    )
    .fetch_all(&app_state.db)
    .await?;

    Ok(Json(report_types))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewReportType>,
) -> Result<(StatusCode, Json<ReportType>), ApiError> {
    check_permission(user.role.report_create)?;

    let report_type = query_as!(
        ReportType,
        r#"
        INSERT INTO
            report_types
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
    .await?;

    Ok((StatusCode::CREATED, Json(report_type)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateReportType>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.report_edit)?;

    let result = query!(
        r#"
        UPDATE 
            report_types rt
        SET
            name = $1
        WHERE
            rt.id = $2
        "#,
        body.name,
        body.id
    )
    .execute(&app_state.db)
    .await?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryReportType>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.report_delete)?;

    let result = query!(r#"DELETE FROM report_types WHERE id = $1"#, params.id)
        .execute(&app_state.db)
        .await?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
