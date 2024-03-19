use crate::{auth::auth, machine, task, user, AppState};
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
        .route("/machine", get(machine::details))
        .route("/machines", get(machine::index))
        .route("/machine", post(machine::create))
        .route("/machine", delete(machine::delete))
        .route("/machine", put(machine::update))
        .route("/user/me", get(user::me))
        .route("/user", get(user::details))
        .route("/users", get(user::index))
        .route("/user", post(user::create))
        .route("/user", put(user::update))
        .route("/user/logout", get(user::logout))
        .route("/task", get(task::details))
        .route("/tasks", get(task::index))
        .route("/task", post(task::create))
        .route("/task", delete(task::delete))
        .route("/task", put(task::update))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth));

    let api = Router::new()
        .route("/user/internal/login", post(user::login_internal))
        .route("/user/login", post(user::login_initiate))
        .route("/user/external/verify", post(user::verify_external))
        .nest("/auth", auth);

    let app = Router::new()
        .nest("/api", api)
        .route("/assets/*path", get(file_serve))
        .route("/", index_serve.to_owned())
        .route("/*path", index_serve)
        .with_state(app_state);

    app
}
