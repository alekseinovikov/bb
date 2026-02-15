use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use fs2::FileExt;

pub struct DaemonLockGuard {
    file: File,
    _path: PathBuf,
}

impl Drop for DaemonLockGuard {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

pub fn is_lock_held_exclusively(path: &Path) -> anyhow::Result<bool> {
    let file = open_lock_file(path)?;
    match file.try_lock_exclusive() {
        Ok(()) => {
            file.unlock()?;
            Ok(false)
        }
        Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => Ok(true),
        Err(err) => Err(err.into()),
    }
}

pub fn acquire_daemon_lock(path: &Path) -> anyhow::Result<DaemonLockGuard> {
    let file = open_lock_file(path)?;
    match file.try_lock_exclusive() {
        Ok(()) => Ok(DaemonLockGuard {
            file,
            _path: path.to_path_buf(),
        }),
        Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
            anyhow::bail!("daemon lock is already held: {}", path.display())
        }
        Err(err) => Err(err.into()),
    }
}

fn open_lock_file(path: &Path) -> anyhow::Result<File> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    Ok(file)
}
