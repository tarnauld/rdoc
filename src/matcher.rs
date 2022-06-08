use crate::init;
use clap::ArgMatches;

pub fn match_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("init", _matches)) => init::init_project(),
        _ => std::process::exit(0)
    }
}