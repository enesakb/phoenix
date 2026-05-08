use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize Phoenix's tracing subscriber.
///
/// Logs to both stdout and `<log_dir>/phoenix.log`. Returns a guard that must
/// remain in scope for the lifetime of the process — dropping it flushes
/// pending log lines.
pub fn init(log_dir: &Path) -> std::io::Result<WorkerGuard> {
    std::fs::create_dir_all(log_dir)?;
    let file_appender = tracing_appender::rolling::daily(log_dir, "phoenix.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let stdout_layer = fmt::layer().with_target(true).with_level(true);
    let file_layer = fmt::layer().with_writer(non_blocking).json();

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .with(file_layer)
        .try_init();

    Ok(guard)
}

/// Test-only initializer that uses a tempdir and ignores re-init failures.
pub fn init_for_tests() -> Option<WorkerGuard> {
    let tmp = std::env::temp_dir().join("phoenix-test-logs");
    init(&tmp).ok()
}
