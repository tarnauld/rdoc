use crate::utils::file;
use clap::{Arg, Command};

pub fn init_show_command<'index>() -> Command<'index> {
    return Command::new("show")
        .about("Show commit info.")
        .arg(Arg::new("commit"));
}

pub fn show(commit_uid: Option<&str>) {
    let commits = file::get_commits_from_index();

    commits.iter().for_each(|commit| {
        if commit.id == commit_uid.expect("commit should not be empty") {
            let c = serde_json::to_string_pretty(commit).expect("not found");
            println!("{}", c);
        }
    });
}
