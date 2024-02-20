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
    auth::auth,
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api = Router::new()

        .route("/machine", get(machine::details))
        .route("/machines", get(machine::index))
        .route("/machine", post(machine::create))
        .route("/machine", delete(machine::delete))
        .route("/machine", put(machine::update));
    
    let app = Router::new()
    
        .nest("/api", api)
        .with_state(app_state);

    app
}