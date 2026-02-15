use serde::{Serialize, de::DeserializeOwned};

pub async fn send_json<T: Serialize>(_value: &T) -> anyhow::Result<()> {
    // Transport framing will be implemented in a later phase.
    Ok(())
}

pub async fn recv_json<T: DeserializeOwned>() -> anyhow::Result<T> {
    anyhow::bail!("transport scaffold: recv_json is not implemented")
}
