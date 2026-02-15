use tracing::info;

use crate::config::RuntimeConfig;

pub async fn run_daemon(config: &RuntimeConfig) -> anyhow::Result<()> {
    info!(
        socket = %config.paths.socket_path.display(),
        pid = %config.paths.pid_path.display(),
        "daemon mode scaffold"
    );

    // Intentionally left as scaffolding only.
    Ok(())
}
