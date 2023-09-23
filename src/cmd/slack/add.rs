use crate::base::Cmd;
use clap::{Arg, ArgMatches, Command};

pub struct SubCommandAdd;

const ID_NAME: &str = "NAME";
const ID_URL: &str = "URL";
const ID_MEMO: &str = "MEMO";

impl Cmd for SubCommandAdd {
    const NAME: &'static str = "add";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("add slack webhook")
            .arg(Arg::new(ID_NAME).required(true).help("slack webhook name"))
            .arg(
                Arg::new(ID_URL)
                    .required(true)
                    .help("slack incoming webhook url"),
            )
            .arg(Arg::new(ID_MEMO).required(false).short('m').long("memo"))
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let name: &String = args.get_one(ID_NAME).unwrap();
        let url: &String = args.get_one(ID_URL).unwrap();
        let memo: Option<String> = args.get_one(ID_MEMO).map(|f: &String| f.to_string());

        let mut config = crate::models::SlackWebhookConfig::load()?;
        config.add(name, url, &memo);
        config.save()?;
        Ok(())
    }
}
