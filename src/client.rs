use std::time::Duration;

use tokio::io::AsyncReadExt;
use tracing::info;

use crate::config::RuntimeConfig;
use crate::daemon::{lock, spawn};
use crate::ipc::socket;

pub async fn run_client(config: &RuntimeConfig) -> anyhow::Result<()> {
    config.paths.ensure_runtime_dir()?;

    info!(
        socket = %config.paths.socket_path.display(),
        lock = %config.paths.lock_path.display(),
        pid = %config.paths.pid_path.display(),
        ping = config.ping,
        "client mode start"
    );

    let lock_is_held = lock::is_lock_held_exclusively(&config.paths.lock_path)?;
    if !lock_is_held {
        let binary = std::env::current_exe()?;
        spawn::spawn_daemon(&binary, &config.paths.socket_path, &config.paths.pid_path)?;
    }

    wait_for_daemon_socket(config).await?;

    if config.ping {
        println!("ok");
        return Ok(());
    }

    info!("daemon is reachable; request flow implementation is pending");
    Ok(())
}

async fn wait_for_daemon_socket(config: &RuntimeConfig) -> anyhow::Result<()> {
    const ATTEMPTS: usize = 30;
    const DELAY_MS: u64 = 100;

    for _ in 0..ATTEMPTS {
        if let Ok(Ok(mut stream)) = tokio::time::timeout(
            Duration::from_millis(DELAY_MS),
            socket::connect(&config.paths.socket_path),
        )
        .await
        {
            let mut buffer = [0_u8; 16];
            let _ = stream.read(&mut buffer).await?;
            return Ok(());
        }

        tokio::time::sleep(Duration::from_millis(DELAY_MS)).await;
    }

    anyhow::bail!(
        "daemon lock/socket check failed: lock_path={}, socket_path={}",
        config.paths.lock_path.display(),
        config.paths.socket_path.display()
    );
}
