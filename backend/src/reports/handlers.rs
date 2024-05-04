use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::{query, query_as, query_scalar, Postgres, QueryBuilder};

use crate::{
    field_vec,
    machines::models::ShortMachine,
    update_field,
    users::models::{ShortUser, User},
    utils::{
        check_permission,
        db::{Field, IntoField},
        errors::{ApiError, ForbiddenReason, InputInvalidReason},
    },
    AppState,
};

use super::{
    models::{DeleteReport, NewReport, QueryReport, Report, UpdateReport},
    report_documents::ReportDocument,
    report_statuses::ReportStatus,
    report_types::ReportType,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryReport>,
) -> Result<Json<Vec<Report>>, ApiError> {
    let user_id = user.id;

    let permissions_ok =
        user.role.report_view || params.creator_id.map_or(false, |id| id == user_id);

    if !permissions_ok {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

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
                m.id,
                m.name,
                m.make,
                m.image
            ) AS "machine?: ShortMachine",
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
        LEFT JOIN
            machines m
        ON
            r.machine = m.id
         WHERE
            ($1::UUID IS NULL OR r.id = $1)
        AND
            ($2::UUID IS NULL OR r.creator = $2)
        "#,
        params.report_id,
        params.creator_id
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(reports))
}

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
                m.id,
                m.name,
                m.make,
                m.image
            ) AS "machine?: ShortMachine",
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
        LEFT JOIN
            machines m
        ON
            r.machine = m.id
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

    let report = query_as!(
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
                m.id,
                m.name,
                m.make,
                m.image
            ) AS "machine?: ShortMachine",
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
        LEFT JOIN
            machines m
        ON
            r.machine = m.id
        WHERE
            r.id = $1
        "#,
        report_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(ApiError::from)?;

    tx.commit().await.map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(report)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateReport>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.report_edit)?;

    let mut query_builder = QueryBuilder::<Postgres>::new("UPDATE reports SET");
    let mut separated_list = query_builder.separated(",");

    let fields = field_vec!(
        title => body.title,
        description => body.description,
        report_type => body.report_type,
        status => body.status,
        archived => body.archived,
        machine => body.machine
    );

    if fields.len() < 1 {
        return Err(ApiError::InputInvalid(InputInvalidReason::NoFieldsToUpdate));
    }

    for (field, value) in fields {
        update_field!(separated_list, field, value);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(body.id);

    let result = query_builder
        .build()
        .execute(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<DeleteReport>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.report_delete)?;

    let result = query!(
        r#"
        DELETE FROM
            reports r
        WHERE
            r.id = $1
        "#,
        params.id
    )
    .execute(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
