use crate::commands::{index, init, tag};
use clap::ArgMatches;

pub fn match_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("init", _)) => init::init_project(),
        Some(("index", _)) => index::create_index(),
        Some(("tag", args)) => tag::create_tag(args.value_of("commit"), args.value_of("tag")),
        _ => std::process::exit(0),
    }
}
