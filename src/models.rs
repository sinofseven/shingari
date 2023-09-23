use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackWebhook {
    pub name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackWebhookConfig {
    pub slack_webhooks: Vec<SlackWebhook>,
}

impl SlackWebhookConfig {
    pub fn load() -> Result<SlackWebhookConfig, String> {
        let path = crate::path::get_path_file_slack_webhook()?;
        if !path.exists() {
            return Ok(SlackWebhookConfig {
                slack_webhooks: Vec::new(),
            });
        }
        let text = crate::fs::load_text(&path)?;
        serde_json::from_str(&text).map_err(|e| {
            format!(
                "failed to deserialize config slack webhook: path={}, err={}",
                path.display(),
                e
            )
        })
    }

    pub fn save(&self) -> Result<(), String> {
        let text = serde_json::to_string_pretty(self)
            .map_err(|e| format!("failed to serialize config slack webhook: {e}"))?;
        let path = crate::path::get_path_file_slack_webhook()?;
        crate::fs::save_text(&path, &text)
    }

    pub fn get(&self, name: &str) -> Option<&SlackWebhook> {
        self.slack_webhooks.iter().find(|i| i.name == name)
    }

    pub fn add(&mut self, name: &str, url: &str, memo: &Option<String>) {
        self.slack_webhooks.push(SlackWebhook {
            name: name.to_string(),
            url: url.to_string(),
            memo: memo.clone(),
        });
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        if self.get(name).is_some() {
            /*
            self.slack_webhooks = self
                .slack_webhooks
                .iter()
                .filter(|i| i.name != name)
                .map(|i| i.clone())
                .collect();

             */
            /*
            self.slack_webhooks = self
                .slack_webhooks
                .iter()
                .filter(|i| i.name != name)
                .cloned()
                .collect();

             */
            self.slack_webhooks.retain(|i| i.name != name);
            Ok(())
        } else {
            Err(format!("not found slack webhook (name: {name})"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringTarget {
    pub pid: i32,
    pub webhook_url: String,
    pub memo: Option<String>,
    pub cwd: String,
    pub cmdline: String,
    pub created_at: u128,
    pub updated_at: Option<u128>,
}

impl MonitoringTarget {
    pub fn new(
        pid: &i32,
        webhook_url: &str,
        memo: &Option<String>,
    ) -> Result<MonitoringTarget, String> {
        let (cwd, cmdline) =
            crate::process::get_process(pid)?.ok_or(format!("not found process (pid: {pid})"))?;
        Ok(MonitoringTarget {
            pid: *pid,
            webhook_url: webhook_url.to_string(),
            memo: memo.clone(),
            cwd,
            cmdline,
            created_at: crate::time::now()?,
            updated_at: None,
        })
    }

    pub fn load(pid: &i32) -> Result<MonitoringTarget, String> {
        let path = crate::path::get_path_file_target(pid)?;
        let text = crate::fs::load_text(&path)?;
        serde_json::from_str(&text)
            .map_err(|e| format!("failed to deserialize monitoring target file: {e}"))
    }

    pub fn save_target(&self) -> Result<(), String> {
        let text = serde_json::to_string_pretty(self)
            .map_err(|e| format!("failed to serialize monitoring target: {e}"))?;
        let path = crate::path::get_path_file_target(&self.pid)?;
        crate::fs::save_text(&path, &text)
    }

    pub fn delete_target(&self) -> Result<(), String> {
        let path = crate::path::get_path_file_target(&self.pid)?;
        crate::fs::delete_file(&path)
    }

    pub fn save_history(&self) -> Result<(), String> {
        let data = MonitoringTarget {
            pid: self.pid,
            webhook_url: self.webhook_url.to_string(),
            memo: self.memo.clone(),
            cwd: self.cwd.to_string(),
            cmdline: self.cmdline.to_string(),
            created_at: self.created_at,
            updated_at: Some(crate::time::now()?),
        };
        let text = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("failed to serialize monitoring target: {e}"))?;
        let rtid = crate::rtid::generate_rtid()?;
        let path = crate::path::get_path_file_history(&rtid)?;
        crate::fs::save_text(&path, &text)
    }
}
