use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> Result<u128, String> {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .map_err(|e| format!("failed to get unixtime: {e}"))
}
