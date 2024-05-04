use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Extension, Json};
use sqlx::{query, query_as, query_scalar};

use crate::{
    users::models::{ShortUser, User},
    utils::{check_permission, errors::ApiError},
    AppState,
};

use super::{
    models::{NewReport, Report},
    report_documents::ReportDocument,
    report_statuses::ReportStatus,
    report_types::ReportType,
};

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Report>>, ApiError> {
    check_permission(user.role.report_view)?;

    let reports = query_as!(
        Report,
        r#"
        SELECT
            r.id,
            r.title,
            r.description,
            (
                rt.id,
                rt.name
            ) AS "report_type!: ReportType",
            (
                rs.id,
                rs.name
            ) AS "status!: ReportStatus",
            r.archived,
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
                        rd.uri,
                        rd.name,
                        rd.description
                    )
                )
                FROM report_documents rd
                WHERE rd.report_id = r.id
            ) AS "documents: Vec<ReportDocument>",
            r.created,
            r.edited
        FROM
            reports r
        INNER JOIN
            report_types rt
        ON
            r.report_type = rt.id
        INNER JOIN
            report_statuses rs
        ON
            r.status = rs.id
        INNER JOIN
            users u
        ON
            r.creator = u.id
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(reports))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewReport>,
) -> Result<(StatusCode, Json<Report>), ApiError> {
    check_permission(user.role.report_create)?;

    let mut tx = app_state.db.begin().await.map_err(ApiError::from)?;

    let report_id = query_scalar!(
        r#"
        INSERT INTO
            reports
        (
            title,
            description,
            report_type,
            status,
            archived,
            creator
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6
        )
        RETURNING
            id
        "#,
        body.title,
        body.description,
        body.report_type,
        body.status,
        body.archived.unwrap_or(false),
        user.id,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(ApiError::from)?;

    tx.commit().await.map_err(ApiError::from)?;

    todo!()
}
