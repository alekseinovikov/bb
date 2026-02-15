use crate::config::RuntimeConfig;

pub async fn serve(_config: &RuntimeConfig) -> anyhow::Result<()> {
    // Daemon runtime implementation is intentionally deferred.
    Ok(())
}
