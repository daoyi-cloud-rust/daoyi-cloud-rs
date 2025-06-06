use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub use tracing::debug;
pub use tracing::error;
pub use tracing::info;
pub use tracing::warn;
use tracing_subscriber::fmt::time::ChronoLocal;

pub fn init(directives: Option<&str>) {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(directives.unwrap_or_else(|| "info"))),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(ChronoLocal::rfc_3339())
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(false),
        )
        .init();
}
