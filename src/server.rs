use tracing::{info, warn};

use crate::config::RuntimeConfig;
use crate::daemon::{lock, runtime};

pub async fn run_daemon(config: &RuntimeConfig) -> anyhow::Result<()> {
    config.paths.ensure_runtime_dir()?;

    info!(
        socket = %config.paths.socket_path.display(),
        lock = %config.paths.lock_path.display(),
        pid = %config.paths.pid_path.display(),
        "daemon mode starting"
    );

    let _lock_guard = match lock::acquire_daemon_lock(&config.paths.lock_path) {
        Ok(guard) => guard,
        Err(err) => {
            warn!(error = %err, "daemon lock is already held, exiting");
            return Ok(());
        }
    };

    std::fs::write(&config.paths.pid_path, format!("{}\n", std::process::id()))?;
    let serve_result = runtime::serve(config).await;

    if config.paths.socket_path.exists() {
        let _ = std::fs::remove_file(&config.paths.socket_path);
    }
    if config.paths.pid_path.exists() {
        let _ = std::fs::remove_file(&config.paths.pid_path);
    }

    serve_result
}
