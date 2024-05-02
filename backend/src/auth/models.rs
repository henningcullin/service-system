use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct LoginToken {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub hash: String,
}

#[derive(Serialize)]
pub enum LoginKind {
    OTP,
    Password,
}

#[derive(Validate, Deserialize)]
pub struct LoginEmail {
    pub email: String,
}
