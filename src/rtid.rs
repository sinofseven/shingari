use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_rtid() -> Result<String, String> {
    let now = SystemTime::now();
    let unixtime = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("failed to get unixtime: {e}"))?;
    let max_number: u128 = 9007199254740991;
    let rtid = max_number - unixtime.as_millis();

    Ok(format!("{:016}", rtid))
}
