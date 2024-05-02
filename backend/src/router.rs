use crate::{/* auth::auth, machine, task, user, */ machines::{self, facilities}, users::{self, roles}, AppState};
use axum::{
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
        // Users
        .route("/user", get(users::details))
        .route("/users", get(users::index))
        
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
        .route("/machines", get(machines::index));


    let api = Router::new()
        .nest("/auth", auth);

    let app = Router::new()
        .nest("/api", api)
        .route("/assets/*path", get(file_serve))
        .route("/", index_serve.to_owned())
        .route("/*path", index_serve)
        .with_state(app_state);

    app
}
