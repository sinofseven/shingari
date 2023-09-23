use crate::base::Cmd;
use clap::{Arg, ArgMatches, Command};

pub struct SubCommandRemove;

const ID_PID: &str = "PID";

impl Cmd for SubCommandRemove {
    const NAME: &'static str = "remove";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("remove monitoring target waiting process finish")
            .arg(Arg::new(ID_PID).required(true))
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let pid: &String = args.get_one(ID_PID).unwrap();
        let pid: i32 = pid
            .parse()
            .map_err(|e| format!("failed to convert pid to int: pid={pid}, err={e}"))?;
        let item = crate::models::MonitoringTarget::load(&pid)?;
        item.delete_target()?;
        Ok(())
    }
}
