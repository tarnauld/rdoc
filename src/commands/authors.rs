use clap::Arg;
use clap::Command;
use crate::utils::file;
use crate::models::commit;

pub fn init_authors_command<'authors>() -> Command<'authors> {
    return Command::new("authors")
        .about("Add authors for a commit")
        .arg(Arg::new("commit").required(true))
        .arg(Arg::new("authors").required(true));
}

pub fn update_authors(commit: Option<&str>, authors: Option<&str>) {
    let commit_id = get_option(commit);
    let authors = get_option(authors);

    let mut commits: Vec<commit::CommitInfo> = file::get_commits_from_index();
    commits.iter_mut().for_each(|commit| {
        if commit.id == commit_id {
            commit.update_authors(String::from(authors));
        }
    });

    match file::write_file(commits) {
        Ok(_) => (),
        Err(_) => ()
    };
}

fn get_option(option: Option<&str>) -> &str {
    match option {
        Some(e) => e,
        None => panic!()
    }
}