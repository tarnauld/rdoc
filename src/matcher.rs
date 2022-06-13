use crate::commands::{authors, describe, generate, index, init, search, tag, update};
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
        Some(("search", args)) => search::search(
            args.value_of("terms"),
            args.is_present("all"),
            args.is_present("author"),
            args.is_present("tag"),
        ),
        _ => std::process::exit(0),
    }
}
