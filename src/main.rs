pub mod base;
pub mod cmd;
pub mod fs;
pub mod glob;
pub mod models;
pub mod my_log;
pub mod path;
pub mod process;
pub mod rtid;
pub mod slack;
pub mod time;

use base::Cmd;
use clap::command;
use log::error;

use cmd::{SubCommandCheck, SubCommandProcess, SubCommandSlack};

fn main() -> Result<(), String> {
    my_log::init_log()?;
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(SubCommandProcess::subcommand())
        .subcommand(SubCommandSlack::subcommand())
        .subcommand(SubCommandCheck::subcommand())
        .get_matches();

    let result = match matches.subcommand() {
        Some((SubCommandProcess::NAME, args)) => SubCommandProcess::run(args),
        Some((SubCommandSlack::NAME, args)) => SubCommandSlack::run(args),
        Some((SubCommandCheck::NAME, args)) => SubCommandCheck::run(args),
        _ => unreachable!(""),
    };

    if let Err(msg) = &result {
        error!("{msg}");
    }

    result
}
