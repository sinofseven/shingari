use crate::base::Cmd;
use clap::{ArgMatches, Command};

pub struct SubCommandList;

impl Cmd for SubCommandList {
    const NAME: &'static str = "list";

    fn subcommand() -> Command {
        Command::new(Self::NAME).about("list slack webhook names")
    }

    fn run(_args: &ArgMatches) -> Result<(), String> {
        let config = crate::models::SlackWebhookConfig::load()?;
        let names: Vec<&String> = config.slack_webhooks.iter().map(|i| &i.name).collect();
        let text = serde_json::to_string_pretty(&names)
            .map_err(|e| format!("failed to serialize webhook names: {e}"))?;
        println!("{text}");
        Ok(())
    }
}
