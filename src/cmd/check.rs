use crate::base::Cmd;
use clap::{ArgMatches, Command};

pub struct SubCommandCheck;

impl Cmd for SubCommandCheck {
    const NAME: &'static str = "check";

    fn subcommand() -> Command {
        Command::new(Self::NAME).about("check processes")
    }

    fn run(_args: &ArgMatches) -> Result<(), String> {
        for pid in crate::glob::get_pid_files()? {
            let info = crate::process::get_process(&pid)?;
            if info.is_some() {
                continue;
            }
            let target = crate::models::MonitoringTarget::load(&pid)?;
            crate::slack::send_slack(&target, false)?;
            target.save_history()?;
            target.delete_target()?;
        }

        Ok(())
    }
}
