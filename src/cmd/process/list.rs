use crate::base::Cmd;
use clap::{ArgMatches, Command};

pub struct SubCommandList;

impl Cmd for SubCommandList {
    const NAME: &'static str = "list";

    fn subcommand() -> Command {
        Command::new(Self::NAME).about("list monitoring targets waiting process finish")
    }

    fn run(_args: &ArgMatches) -> Result<(), String> {
        let pids = crate::glob::get_pid_files()?;
        let text = serde_json::to_string_pretty(&pids)
            .map_err(|e| format!("failed to serialize pids array: {e}"))?;
        println!("{text}");
        Ok(())
    }
}
