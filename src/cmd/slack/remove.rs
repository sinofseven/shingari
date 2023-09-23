use crate::base::Cmd;
use clap::{Arg, ArgMatches, Command};

pub struct SubCommandRemove;
const ID_NAME: &str = "NAME";

impl Cmd for SubCommandRemove {
    const NAME: &'static str = "remove";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("remove slack incoming webhook url")
            .arg(Arg::new(ID_NAME).required(true).help("slack webhook name"))
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let name: &String = args.get_one(ID_NAME).unwrap();
        let mut config = crate::models::SlackWebhookConfig::load()?;
        config.remove(name)?;
        config.save()?;
        Ok(())
    }
}
