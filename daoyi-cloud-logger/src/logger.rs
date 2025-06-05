use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

pub use tracing::debug;
pub use tracing::error;
pub use tracing::info;
pub use tracing::warn;

pub fn init(directives: Option<&str>) {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(directives.unwrap_or_else(|| "info"))),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(fmt::time::OffsetTime::new(
                    time::OffsetDateTime::now_local().unwrap().offset(),
                    time::format_description::parse(
                        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]",
                    )
                    .unwrap(),
                ))
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(false),
        )
        .init();
}
