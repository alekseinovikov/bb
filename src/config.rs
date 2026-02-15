use std::path::PathBuf;

use crate::shell::ShellKind;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Client,
    Daemon,
}

#[derive(Debug)]
pub struct RuntimePaths {
    pub runtime_dir: PathBuf,
    pub socket_path: PathBuf,
    pub pid_path: PathBuf,
    pub lock_path: PathBuf,
}

#[derive(Debug)]
pub struct RuntimeConfig {
    pub mode: Mode,
    pub ping: bool,
    pub paths: RuntimePaths,
    pub shell: Option<ShellKind>,
}

impl RuntimeConfig {
    pub fn new(
        mode: Mode,
        ping: bool,
        socket_override: Option<PathBuf>,
        pid_override: Option<PathBuf>,
        shell: Option<ShellKind>,
    ) -> anyhow::Result<Self> {
        let paths = RuntimePaths::resolve(socket_override, pid_override)?;
        Ok(Self {
            mode,
            ping,
            paths,
            shell,
        })
    }
}

impl RuntimePaths {
    pub fn ensure_runtime_dir(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.runtime_dir)?;
        Ok(())
    }

    fn resolve(
        socket_override: Option<PathBuf>,
        pid_override: Option<PathBuf>,
    ) -> anyhow::Result<Self> {
        let runtime_dir = default_runtime_dir()?;
        let socket_path = socket_override.unwrap_or_else(|| runtime_dir.join("bb.sock"));
        let pid_path = pid_override.unwrap_or_else(|| runtime_dir.join("bb.pid"));
        let lock_path = runtime_dir.join("bb.lock");

        Ok(Self {
            runtime_dir,
            socket_path,
            pid_path,
            lock_path,
        })
    }
}

fn default_runtime_dir() -> anyhow::Result<PathBuf> {
    if let Some(xdg_runtime) = std::env::var_os("XDG_RUNTIME_DIR") {
        return Ok(PathBuf::from(xdg_runtime).join("bb"));
    }

    let uid = nix_like_uid();
    Ok(std::env::temp_dir().join(format!("bb-{uid}")))
}

fn nix_like_uid() -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;

        if let Ok(meta) = std::fs::metadata(".") {
            return meta.uid();
        }
    }

    0
}
