pub mod base;
pub mod cmd;
pub mod fs;
pub mod glob;
pub mod models;
pub mod path;
pub mod process;
pub mod rtid;
pub mod slack;
pub mod time;

use base::Cmd;
use clap::command;

use cmd::{SubCommandCheck, SubCommandProcess, SubCommandSlack};

fn main() -> Result<(), String> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(SubCommandProcess::subcommand())
        .subcommand(SubCommandSlack::subcommand())
        .subcommand(SubCommandCheck::subcommand())
        .get_matches();

    match matches.subcommand() {
        Some((SubCommandProcess::NAME, args)) => SubCommandProcess::run(args),
        Some((SubCommandSlack::NAME, args)) => SubCommandSlack::run(args),
        Some((SubCommandCheck::NAME, args)) => SubCommandCheck::run(args),
        _ => unreachable!(""),
    }
}
