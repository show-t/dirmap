use std::path::PathBuf;

use anyhow::Result;
use chrono::Local;
use dotenvy;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{self, fmt, prelude::*, EnvFilter, Registry};
use tracing_subscriber::fmt::time::LocalTime;
use tracing_appender::{rolling, non_blocking::WorkerGuard};

fn log_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    let base = std::env::var("USERPROFILE")
        .map(|s| s + r"\logs")
        .unwrap_or_else(|_| r"C:\Users\Default\logs".to_string());

    #[cfg(target_os = "macos")]
    let base = r"/usr/local/var/logs";

    #[cfg(target_os = "linux")]
    let base = r"/var/logs";

    PathBuf::from(base).join("dirmap")
}

pub(crate) struct TracingGuard {
    _guard: WorkerGuard,
}

pub(crate) fn tracing_init() -> Result<TracingGuard> {
    dotenvy::dotenv()?;

    // EnvFilter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Stdout Layer
    let stdout_layer = fmt::layer()
        .with_timer(LocalTime::rfc_3339())
        .with_writer(std::io::stdout)
        .with_filter(env_filter);

    // File Layer
    let filename = format!("error_{}.log", Local::now().format("%Y%m%d"));
    let file_appender = rolling::never(log_dir(), &filename);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_timer(LocalTime::rfc_3339())
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(LevelFilter::ERROR);

    // Subscriber registration
    Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    Ok(TracingGuard{ _guard })
}