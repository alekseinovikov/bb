use tracing::info;

use crate::config::RuntimeConfig;

pub async fn run_client(config: &RuntimeConfig) -> anyhow::Result<()> {
    info!(
        socket = %config.paths.socket_path.display(),
        pid = %config.paths.pid_path.display(),
        ping = config.ping,
        "client mode scaffold"
    );

    // Intentionally left as scaffolding only.
    Ok(())
}
