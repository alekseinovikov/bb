use std::path::Path;

use tokio::net::{UnixListener, UnixStream};

pub async fn connect(path: &Path) -> std::io::Result<UnixStream> {
    UnixStream::connect(path).await
}

pub async fn bind(path: &Path) -> std::io::Result<UnixListener> {
    UnixListener::bind(path)
}
