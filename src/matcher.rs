use crate::commands::{index, init, tag, authors, generate, describe};
use clap::ArgMatches;

pub fn match_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("init", _)) => init::init_project(),
        Some(("index", _)) => index::create_index(),
        Some(("tag", args)) => tag::create_tag(args.value_of("commit"), args.value_of("tag")),
        Some(("generate", _)) => generate::generate(),
        Some(("authors", args)) => authors::update_authors(args.value_of("commit"), args.value_of("authors")),
        Some(("describe", args)) => describe::update_description(args.value_of("commit")),
        _ => std::process::exit(0),
    }
}
