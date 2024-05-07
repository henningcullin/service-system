use crate::{
    auth::{self, auth},
    channels,
    machines::{self, facilities, machine_statuses, machine_types},
    reports::{self, report_statuses, report_types},
    tasks::{self, task_executors, task_statuses, task_types},
    users::{self, roles},
    AppState,
};
use axum::{
    middleware,
    routing::{delete, get, get_service, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let file_serve = get_service(ServeDir::new(&app_state.env.frontend_url));

    let index_serve = get_service(ServeFile::new(
        app_state.env.frontend_url.to_owned() + "\\index.html",
    ));

    let channels = Router::new()
        .route("/tasks", get(channels::task_listen))
        .route("/reports", get(channels::report_listen));

    let auth = Router::new()
        .nest("/channel", channels)
        // Auth
        .route("/logout", get(auth::logout))
        .route("/me", get(auth::me))
        // Users
        .route("/user", get(users::details))
        .route("/users", get(users::index))
        .route("/user", post(users::create))
        .route("/user", put(users::update))
        // Roles
        .route("/role", get(roles::details))
        .route("/roles", get(roles::index))
        .route("/role", post(roles::create))
        .route("/role", put(roles::update))
        .route("/role", delete(roles::delete))
        // Reports
        .route("/report", get(reports::details))
        .route("/reports", get(reports::index))
        .route("/report", post(reports::create))
        .route("/report", put(reports::update))
        .route("/report", delete(reports::delete))
        // ReportTypes
        .route("/report_type", get(report_types::details))
        .route("/report_types", get(report_types::index))
        .route("/report_type", post(report_types::create))
        .route("/report_type", put(report_types::update))
        .route("/report_type", delete(report_types::delete))
        // ReportStauses
        .route("/report_status", get(report_statuses::details))
        .route("/report_statuses", get(report_statuses::index))
        .route("/report_status", post(report_statuses::create))
        .route("/report_status", put(report_statuses::update))
        .route("/report_status", delete(report_statuses::delete))
        // Tasks
        .route("/task", get(tasks::details))
        .route("/tasks", get(tasks::index))
        .route("/task", post(tasks::create))
        .route("/task", put(tasks::update))
        .route("/task", delete(tasks::delete))
        // TaskTypes
        .route("/task_type", get(task_types::details))
        .route("/task_types", get(task_types::index))
        .route("/task_type", post(task_types::create))
        .route("/task_type", put(task_types::update))
        .route("/task_type", delete(task_types::delete))
        // TaskStatuses
        .route("/task_status", get(task_statuses::details))
        .route("/task_statuses", get(task_statuses::index))
        .route("/task_status", post(task_statuses::create))
        .route("/task_status", put(task_statuses::update))
        .route("/task_status", delete(task_statuses::delete))
        // TaskExecutors
        .route("/task_executor", post(task_executors::create))
        .route("/task_executor", delete(task_executors::delete))
        // Facilities
        .route("/facility", get(facilities::details))
        .route("/facilities", get(facilities::index))
        .route("/facility", post(facilities::create))
        .route("/facility", put(facilities::update))
        .route("/facility", delete(facilities::delete))
        // Machines
        .route("/machine", get(machines::details))
        .route("/machines", get(machines::index))
        .route("/machine", post(machines::create))
        .route("/machine", put(machines::update))
        .route("/machine", delete(machines::delete))
        // MachineTypes
        .route("/machine_type", get(machine_types::details))
        .route("/machine_types", get(machine_types::index))
        .route("/machine_type", post(machine_types::create))
        .route("/machine_type", put(machine_types::update))
        .route("/machine_type", delete(machine_types::delete))
        // MachineStauses
        .route("/machine_status", get(machine_statuses::details))
        .route("/machine_statuses", get(machine_statuses::index))
        .route("/machine_status", post(machine_statuses::create))
        .route("/machine_status", put(machine_statuses::update))
        .route("/machine_status", delete(machine_statuses::delete))
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
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    app
}
