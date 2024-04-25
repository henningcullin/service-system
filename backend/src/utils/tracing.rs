use tracing::subscriber;
use tracing_subscriber::fmt;

pub fn init_tracing() {
    let subscriber = fmt::Subscriber::builder()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_target(true)
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("Initializing logging failed");
}