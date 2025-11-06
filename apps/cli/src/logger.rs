use schema_core::TimingsLayer;
use tracing_error::ErrorLayer;

pub fn init_logger() {
    use tracing_subscriber::{EnvFilter, FmtSubscriber, prelude::*};

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_writer(std::io::stderr)
        .finish()
        .with(ErrorLayer::default())
        .with(TimingsLayer);

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|err| eprintln!("Error initializing the global logger: {err}"))
        .ok();
}
