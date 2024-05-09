use tracing::{level_filters::LevelFilter, Level};
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling::daily};
use tracing_subscriber::{fmt, prelude::*, registry::Registry};

pub fn init(log_path: &str) -> (WorkerGuard, WorkerGuard) {
    let (file_appender, file_guard) = non_blocking(daily(log_path, "service_system_log"));
    let file_layer = fmt::Layer::default()
        .with_ansi(false)
        .event_format(fmt::format().with_source_location(true).with_level(true))
        .with_writer(file_appender)
        .with_filter(LevelFilter::from_level(Level::INFO));

    let (terminal_appender, terminal_guard) = non_blocking(std::io::stdout());
    let terminal_layer = fmt::Layer::default()
        .event_format(fmt::format().with_ansi(true).with_level(true))
        .with_writer(terminal_appender)
        .with_filter(LevelFilter::from_level(Level::INFO));

    Registry::default()
        .with(file_layer)
        .with(terminal_layer)
        .init();

    return (file_guard, terminal_guard);
}
