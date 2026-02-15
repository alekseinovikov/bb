use std::path::Path;
use std::process::{Command, Stdio};

pub fn spawn_daemon(binary: &Path, socket_path: &Path, pid_path: &Path) -> anyhow::Result<()> {
    let mut cmd = Command::new(binary);
    cmd.arg("--daemon")
        .arg("--socket")
        .arg(socket_path)
        .arg("--pid-file")
        .arg(pid_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    cmd.spawn()?;
    Ok(())
}
