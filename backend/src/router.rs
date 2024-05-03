use crate::{
    auth::{self, auth},
    machines::{self, facilities},
    users::{self, roles},
    AppState,
};
use axum::{
    middleware,
    routing::{delete, get, get_service, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let file_serve = get_service(ServeDir::new(&app_state.env.frontend_url));

    let index_serve = get_service(ServeFile::new(
        app_state.env.frontend_url.to_owned() + "\\index.html",
    ));

    let auth = Router::new()
        // Auth
        .route("/logout", get(auth::logout))
        .route("/me", get(auth::me))
        // Users
        .route("/user", get(users::details))
        .route("/users", get(users::index))
        .route("/user", post(users::create))
        // Roles
        .route("/role", get(roles::details))
        .route("/roles", get(roles::index))
        .route("/role", post(roles::create))
        .route("/role", put(roles::update))
        .route("/role", delete(roles::delete))
        // Facilities
        .route("/facilities", get(facilities::index))
        // Machines
        .route("/machine", get(machines::details))
        .route("/machines", get(machines::index))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth));

    let api = Router::new()
        .nest("/auth", auth)
        .route("/login", post(auth::login_initiate))
        .route("/login/password", post(auth::login_password))
        .route("/login/otp", post(auth::login_otp));

    let app = Router::new()
        .nest("/api", api)
        .route("/assets/*path", get(file_serve))
        .route("/", index_serve.to_owned())
        .route("/*path", index_serve)
        .with_state(app_state);

    app
}
