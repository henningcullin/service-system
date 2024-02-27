use std::sync::Arc;
use axum::{ // Framework
    routing::{ // HTTP Methods
        get,
        post,
        put,
        delete
    }, 
    Router, // The Router
    middleware
};
use crate::{
    auth::auth, machine, task, user, AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router {

    let auth = Router::new()

        .route("/machine", get(machine::details))
        .route("/machines", get(machine::index))
        .route("/machine", post(machine::create))
        .route("/machine", delete(machine::delete))
        .route("/machine", put(machine::update))
        
        .route("/user", get(user::details))
        .route("/users", get(user::index))
        .route("/user", post(user::create))
        .route("/user", put(user::update))
        .route("/user/logout", get(user::logout))

        .route("/task", get(task::details))
        .route("/tasks", get(task::index))
        .route("/task", post(task::create))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth));

    let api = Router::new()

        .route("/user/internal/login", post(user::login_internal))
        .route("/user/external/login", post(user::login_external))
        .route("/user/external/verify", post(user::verify_external))
        .nest("/auth", auth);
        
    let app = Router::new()
    
        .nest("/api", api)
        .with_state(app_state);

    app
}