use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use rand_core::OsRng;
use sqlx::{query_as, query_scalar, Postgres, QueryBuilder};
use validator::Validate;

use crate::{
    field_vec,
    machines::facilities::Facility,
    update_field, user_from_id,
    utils::{
        check_permission,
        db::{Field, IntoField},
        errors::{ApiError, ConflictReason, ForbiddenReason, InputInvalidReason},
    },
    AppState,
};

use super::{
    models::{NewUser, QueryUser, UpdateUser, User},
    roles::models::Role,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryUser>,
) -> Result<Json<User>, ApiError> {
    check_permission(user.role.user_view)?;

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
    .await?;

    Ok(Json(user))
}

pub async fn index(
    State(app_state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Result<Json<Vec<User>>, ApiError> {
    check_permission(user.role.user_view)?;

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
    .await?;

    Ok(Json(users))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    check_permission(user.role.user_create)?;

    body.validate()?;

    let email = body.email.to_lowercase();

    let user_exists = query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
        .fetch_one(&app_state.db)
        .await?;

    if let Some(exists) = user_exists {
        if exists {
            return Err(ApiError::Conflict(ConflictReason::EmailTaken));
        }
    }

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
        body.role
    )
    .fetch_one(&app_state.db)
    .await?;

    if role.level <= user.role.level {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

    let password = match role.has_password {
        false => None,
        true => match body.password {
            None => {
                return Err(ApiError::InputInvalid(
                    InputInvalidReason::NoPasswordSupplied,
                ))
            }
            Some(password) => {
                let salt = SaltString::generate(&mut OsRng);
                let hashed_password = Argon2::default()
                    .hash_password(password.as_bytes(), &salt)
                    .map(|hash| hash.to_string())?;
                Some(hashed_password)
            }
        },
    };

    let user = query_as!(
        User,
        r#"
        WITH new_user AS (
            INSERT INTO users
            (
                first_name,
                last_name,
                email,
                password,
                phone,
                role,
                active,
                occupation,
                facility
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
                $9
            )
            RETURNING *
        )
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
            new_user u
        INNER JOIN
            roles r
        ON
            u.role = r.id
        LEFT JOIN
            facilities f
        ON
            u.facility = f.id
        "#,
        body.first_name,
        body.last_name,
        email,
        password,
        body.phone,
        body.role,
        body.active.unwrap_or(true),
        body.occupation,
        body.facility,
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateUser>,
) -> Result<StatusCode, ApiError> {
    let permissions_ok = user.role.user_edit || body.id == user.id;

    if !permissions_ok {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

    if body.email.is_some() {
        body.validate()?;
    }

    let target_user = user_from_id!(body.id).fetch_one(&app_state.db).await?;

    if target_user.role.level <= user.role.level && target_user.id != user.id {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

    if let Some(role_id) = body.role {
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
            role_id
        )
        .fetch_one(&app_state.db)
        .await?;

        if role.level <= user.role.level {
            return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
        }
    }

    let mut query_builder = QueryBuilder::<Postgres>::new("UPDATE users SET");
    let mut separated_list = query_builder.separated(",");

    let fields = field_vec![
        first_name => body.first_name,
        last_name => body.last_name,
        email => body.email,
        password => body.password,
        phone => body.phone,
        role => body.role,
        active => body.active,
        occupation => body.occupation,
        facility => body.facility
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
