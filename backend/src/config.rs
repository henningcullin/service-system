#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_pwl_secret: String,
    pub jwt_expires_in: i64,
    pub jwt_maxage: i32,
    pub frontend_url: String,
    pub log_path: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_pwl_secret = std::env::var("JWT_PWL_SECRET").expect("JWT_PWL_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
        let log_path = std::env::var("LOG_PATH").expect("LOG_PATH must be set");
        Config {
            database_url,
            jwt_secret,
            jwt_pwl_secret,
            jwt_expires_in: jwt_expires_in
                .parse::<i64>()
                .expect("Could not parse JWT_EXPIRES_IN to i32"),
            jwt_maxage: jwt_maxage
                .parse::<i32>()
                .expect("Could not parse JWT_MAXAGE to i32"),
            frontend_url,
            log_path,
        }
    }
}
