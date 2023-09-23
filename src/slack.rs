use crate::models::MonitoringTarget;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Payload {
    text: String,
}

#[derive(Debug, Serialize)]
struct Field {
    value: String,
    short: bool,
}

#[derive(Debug, Serialize)]
struct Attachment {
    pretext: String,
    color: String,
    fields: Vec<Field>,
}

#[derive(Debug, Serialize)]
struct PayloadV2 {
    attachments: Vec<Attachment>,
}

fn create_message(target: &MonitoringTarget, is_start: bool) -> Result<String, String> {
    let mut fields: Vec<Field> = vec![
        Field {
            short: false,
            value: (if is_start {
                "start monitoring to wait finish process"
            } else {
                "finish monitoring to wait finish process"
            })
            .to_string(),
        },
        Field {
            short: false,
            value: format!("- pid: `{}`", &target.pid),
        },
        Field {
            short: false,
            value: format!("- cwd: `{}`", &target.cwd),
        },
        Field {
            short: false,
            value: format!("- cmdline: `{}`", &target.cmdline),
        },
    ];

    if let Some(memo) = &target.memo {
        fields.push(Field {
            short: false,
            value: format!("- memo: `{memo}`"),
        })
    }

    let payload = PayloadV2 {
        attachments: vec![Attachment {
            pretext: "<!channel>".to_string(),
            color: (match is_start {
                true => "#AAFFAA",
                false => "#D8A5AA",
            })
            .to_string(),
            fields,
        }],
    };
    serde_json::to_string(&payload).map_err(|e| format!("failed to serialize slack text: {e}"))
}

pub fn send_slack(target: &MonitoringTarget, is_start: bool) -> Result<(), String> {
    let payload = create_message(target, is_start)?;
    ureq::post(&target.webhook_url)
        .set("Content-Type", "application/json")
        .send_string(&payload)
        .map(|_| ())
        .map_err(|e| format!("failed to call slack incoming webhook: {e}"))
}
