use serde::{Deserialize, Serialize};
use validator::Validate;

// Struct for jwt

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

// Temporary jwt token that is used to enable pwl login

#[derive(Serialize, Deserialize)]
pub struct LoginToken {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub hash: String,
}

// Enum which determines which login type the user has

#[derive(Serialize)]
pub enum LoginKind {
    OTP,
    Password,
}

// Initiate login

#[derive(Validate, Deserialize)]
pub struct LoginEmail {
    #[validate(email)]
    pub email: String,
}

#[derive(Validate, Deserialize)]
pub struct LoginPasswordUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginOTPUser {
    pub code: String,
}
