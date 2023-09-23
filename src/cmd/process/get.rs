use crate::base::Cmd;
use clap::{Arg, ArgMatches, Command};

pub struct SubCommandGet;

const ID_PID: &str = "PID";

impl Cmd for SubCommandGet {
    const NAME: &'static str = "get";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("get monitoring target waiting process finish")
            .arg(Arg::new(ID_PID).required(true))
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let pid: &String = args.get_one(ID_PID).unwrap();
        let pid: i32 = pid
            .parse()
            .map_err(|e| format!("failed to convert pid to int: pid={pid}, err={e}"))?;
        let data = crate::models::MonitoringTarget::load(&pid)?;
        let text = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("failed to serialize monitoring target data: {e}"))?;
        println!("{text}");
        Ok(())
    }
}
