use crate::commands::{authors, describe, generate, index, init, tag, update};
use clap::ArgMatches;

pub fn match_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("init", _)) => init::init_project(),
        Some(("index", args)) => index::create_index(args.is_present("force")),
        Some(("tag", args)) => tag::create_tag(args.value_of("commit"), args.value_of("tag")),
        Some(("generate", _)) => generate::generate(),
        Some(("authors", args)) => {
            authors::update_authors(args.value_of("commit"), args.value_of("authors"))
        }
        Some(("describe", args)) => describe::update_description(args.value_of("commit")),
        Some(("update", _)) => update::update_index(),
        _ => std::process::exit(0),
    }
}
