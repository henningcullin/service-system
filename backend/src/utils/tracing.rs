use tracing::Level;
use tracing_subscriber::fmt;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling::daily};

pub fn init_tracing(log_path: &str) -> WorkerGuard {

    let file_appender = daily(log_path, "service_system_log");
    let (non_blocking_appender, guard) = non_blocking(file_appender);

    fmt::Subscriber::builder()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_target(true)
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_writer(non_blocking_appender)
        .init();

    return guard;
}