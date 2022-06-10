use clap::Arg;
use clap::Command;
use crate::utils::file;
use crate::models::commit;
use vim_edit::{vim_create};

pub fn init_describe_command<'describe>() -> Command<'describe> {
    return Command::new("describe")
        .about("Add description for a commit (markdown supported)")
        .arg(Arg::new("commit").required(true));
}

pub fn update_description(commit: Option<&str>) {
    let commit_id = get_option(commit);
    let description: String = vim_create();

    let mut commits: Vec<commit::CommitInfo> = file::get_commits_from_index();
    commits.iter_mut().for_each(|commit| {
        if commit.id == commit_id {
            commit.update_description(&description);
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