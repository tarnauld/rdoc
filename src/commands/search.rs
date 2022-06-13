use clap::Arg;
use clap::Command;
use crate::utils::{repo, file};

pub fn init_search_command<'search>() -> Command<'search> {
    return Command::new("search")
        .about("Returns a list of commits containing expression.")
        .arg(Arg::new("terms").required(true))
        .arg(Arg::new("all").long("all").takes_value(false));
}

pub fn search(terms_opt: Option<&str>, all_opt: bool) {
    let terms = terms_opt.expect("Term should not be empty");

    match all_opt {
        true => search_all(terms),
        false => search_in_message(terms)
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
