use crate::base::Cmd;
use clap::{Arg, ArgMatches, Command};

pub struct SubCommandAdd;

const ID_SLACK_WEBHOOK_NAME: &str = "SLACK_WEBHOOK_NAME";
const ID_PID: &str = "PID";
const ID_MEMO: &str = "MEMO";

impl Cmd for SubCommandAdd {
    const NAME: &'static str = "add";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("add monitoring target waiting process finish")
            .arg(Arg::new(ID_SLACK_WEBHOOK_NAME).required(true))
            .arg(Arg::new(ID_PID).required(true))
            .arg(Arg::new(ID_MEMO).required(false).short('m').long("memo"))
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let slack_webhook_name: &String = args.get_one(ID_SLACK_WEBHOOK_NAME).unwrap();
        let pid: &String = args.get_one(ID_PID).unwrap();
        let pid: i32 = pid
            .parse()
            .map_err(|e| format!("failed to convert pid to int: pid={pid}, err={e}"))?;
        let memo: Option<String> = args.get_one(ID_MEMO).map(|f: &String| f.to_string());

        let config = crate::models::SlackWebhookConfig::load()?;
        let slack_webhook = config.get(slack_webhook_name).ok_or(format!(
            "not found slack webhook (name: {slack_webhook_name})"
        ))?;

        let data = crate::models::MonitoringTarget::new(&pid, &slack_webhook.url, &memo)?;
        data.save_target()?;

        crate::slack::send_slack(&data, true)?;
        Ok(())
    }
}
