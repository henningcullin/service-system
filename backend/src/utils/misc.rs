use super::errors::{ApiError, ForbiddenReason};

pub fn check_permission(permission: bool) -> Result<(), ApiError> {
    if !permission {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }
    Ok(())
}
