use std::path::Path;

pub fn spawn_daemon(_binary: &Path, _socket_path: &Path, _pid_path: &Path) -> anyhow::Result<()> {
    // Daemon auto-spawn implementation is intentionally deferred.
    Ok(())
}
