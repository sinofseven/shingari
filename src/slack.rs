use crate::models::MonitoringTarget;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Payload {
    text: String,
}

fn create_message(target: &MonitoringTarget, is_start: bool) -> Result<String, String> {
    let mut lines: Vec<String> = vec![
        "<!channel>".to_string(),
        (if is_start {
            "*start* monitoring to wait finish process"
        } else {
            "*finish* monitoring to wait finish process"
        })
        .to_string(),
        format!("- pid: `{}`", &target.pid),
        format!("- cwd: `{}`", &target.cwd),
        format!("- cmdline: `{}`", &target.cmdline),
    ];

    if let Some(memo) = &target.memo {
        lines.push(format!("- memo: `{memo}`"));
    }

    let payload = Payload {
        text: lines.join("\n"),
    };
    serde_json::to_string(&payload).map_err(|e| format!("failed to serialize slack text: {e}"))
}

pub fn send_slack(target: &MonitoringTarget, is_start: bool) -> Result<(), String> {
    let payload = create_message(target, is_start)?;
    reqwest::blocking::Client::new()
        .post(&target.webhook_url)
        .body(payload)
        .send()
        .map(|_| ())
        .map_err(|e| format!("failed to send webhook: {e}"))
}
