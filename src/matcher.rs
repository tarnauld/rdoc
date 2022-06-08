use crate::init;
use crate::index;
use clap::ArgMatches;

pub fn match_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("init", _matches)) => init::init_project(),
        Some(("index", _matches)) => index::create_index(),
        _ => std::process::exit(0)
    }
}