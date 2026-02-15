use std::os::unix::fs::PermissionsExt;

use tokio::io::AsyncWriteExt;

use crate::config::RuntimeConfig;
use crate::ipc::socket;

pub async fn serve(config: &RuntimeConfig) -> anyhow::Result<()> {
    if config.paths.socket_path.exists() {
        std::fs::remove_file(&config.paths.socket_path)?;
    }

    let listener = socket::bind(&config.paths.socket_path).await?;
    std::fs::set_permissions(
        &config.paths.socket_path,
        std::fs::Permissions::from_mode(0o600),
    )?;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                break;
            }
            accept_result = listener.accept() => {
                let (mut stream, _) = accept_result?;
                stream.write_all(b"ok\n").await?;
                stream.shutdown().await?;
            }
        }
    }

    Ok(())
}
