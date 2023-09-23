use crate::base::Cmd;
use clap::{Arg, ArgMatches, Command};

pub struct SubCommandGet;

const ID_NAME: &str = "NAME";

impl Cmd for SubCommandGet {
    const NAME: &'static str = "get";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("get slack webhook")
            .arg(Arg::new(ID_NAME))
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let name: &String = args.get_one(ID_NAME).unwrap();
        let config = crate::models::SlackWebhookConfig::load()?;
        if let Some(webhook) = config.get(name) {
            let text = serde_json::to_string_pretty(webhook)
                .map_err(|e| format!("failed to serialize slack webhook: {e}"))?;
            println!("{text}");
            Ok(())
        } else {
            Err(format!("not found slack webhook (name: {name})"))
        }
    }
}
