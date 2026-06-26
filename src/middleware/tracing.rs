use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialise the global tracing subscriber.
///
/// Call once at the very start of `main`.
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,axum_template=debug,tower_http=debug"));

    #[cfg(debug_assertions)]
    let fmt_layer = fmt::layer().pretty();

    #[cfg(not(debug_assertions))]
    let fmt_layer = fmt::layer().json();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}