mod add;
mod get;
mod list;
mod remove;

use crate::base::Cmd;
use add::SubCommandAdd;
use get::SubCommandGet;
use list::SubCommandList;
use remove::SubCommandRemove;

use clap::{ArgMatches, Command};
pub struct SubCommandSlack;

impl Cmd for SubCommandSlack {
    const NAME: &'static str = "slack";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("about slack webhook")
            .subcommand(SubCommandAdd::subcommand())
            .subcommand(SubCommandGet::subcommand())
            .subcommand(SubCommandList::subcommand())
            .subcommand(SubCommandRemove::subcommand())
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            Some((SubCommandAdd::NAME, args)) => SubCommandAdd::run(args),
            Some((SubCommandGet::NAME, args)) => SubCommandGet::run(args),
            Some((SubCommandList::NAME, args)) => SubCommandList::run(args),
            Some((SubCommandRemove::NAME, args)) => SubCommandRemove::run(args),
            _ => SubCommandList::run(args),
        }
    }
}
