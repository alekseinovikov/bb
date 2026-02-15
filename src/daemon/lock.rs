use std::path::Path;

pub struct LockGuard;

pub fn acquire_lock(_path: &Path) -> anyhow::Result<LockGuard> {
    // Locking strategy implementation is intentionally deferred.
    Ok(LockGuard)
}
