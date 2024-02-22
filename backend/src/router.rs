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
    machine,
    user,
    auth::auth,
    AppState,
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
        .route("/user/create", post(user::create))
        .route("/user/update", put(user::update))
        .route("/user/logout", get(user::logout))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth));
    

    let api = Router::new()

        .route("/user/login", post(user::login_internal))
        .nest("/auth", auth);
        
    let app = Router::new()
    
        .nest("/api", api)
        .with_state(app_state);

    app
}