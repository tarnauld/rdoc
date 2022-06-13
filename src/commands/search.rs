use crate::utils::{file, repo};
use clap::Arg;
use clap::Command;

pub fn init_search_command<'search>() -> Command<'search> {
    return Command::new("search")
        .about("Returns a list of commits containing expression.")
        .arg(Arg::new("terms").required(true))
        .arg(
            Arg::new("tag")
                .long("tag")
                .short('t')
                .takes_value(false)
                .long_help("Search by tag"),
        )
        .arg(
            Arg::new("author")
                .long("author")
                .short('w')
                .takes_value(false)
                .long_help("Search by author"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .takes_value(false)
                .long_help("Search in commit message and description"),
        );
}

pub fn search(terms_opt: Option<&str>, all: bool, author: bool, tag: bool) {
    let terms = terms_opt.expect("Term should not be empty");
    let type_of_search = get_type_of_search(all, author, tag);

    match type_of_search {
        TypeOfSearch::All => search_all(terms),
        TypeOfSearch::Author => search_author(terms),
        TypeOfSearch::Tag => search_tag(terms),
        TypeOfSearch::Default => search_in_message(terms),
    }
}

fn search_in_message(terms: &str) {
    let commits = repo::get_commits();
    commits.iter().for_each(|commit| {
        if commit.message.contains(terms) {
            println!("{}: {}", commit.id, commit.message);
        }
    });
}

fn search_all(terms: &str) {
    let commits = file::get_commits_from_index();

    commits.iter().for_each(|commit| {
        if commit.message.contains(terms) || commit.description.contains(terms) {
            println!("{}: {}", commit.id, commit.message);
        }
    });
}

fn get_type_of_search(all: bool, authors: bool, tag: bool) -> TypeOfSearch {
    if all {
        return TypeOfSearch::All;   
    }

    if authors {
        return TypeOfSearch::Author;
    }

    if tag {
        return TypeOfSearch::Tag;
    }

    TypeOfSearch::Default
}

fn search_author(terms: &str) {
    let commits = file::get_commits_from_index();

    commits.iter().for_each(|commit| {
        if commit.authors.contains(terms) {
            println!("{}: {}", commit.id, commit.message);
        }
    });
}

fn search_tag(terms: &str) {
    let commits = file::get_commits_from_index();

    commits.iter().for_each(|commit| {
        if commit.tags.contains(terms) {
            println!("{}: {}", commit.id, commit.message);
        }
    });
}

enum TypeOfSearch {
    Default,
    All,
    Author,
    Tag,
}
