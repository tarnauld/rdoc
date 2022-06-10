use clap::Arg;
use clap::Command;
use crate::utils::file;
use crate::models::commit;

pub fn init_describe_command<'describe>() -> Command<'describe> {
    return Command::new("describe")
        .about("Add description for a commit (markdown supported)")
        .arg(Arg::new("commit").required(true))
        .arg(Arg::new("description").required(true));
}

pub fn update_description(commit: Option<&str>, description: Option<&str>) {
    let commit_id = get_option(commit);
    let description = get_option(description);

    let mut commits: Vec<commit::CommitInfo> = file::get_commits_from_index();
    commits.iter_mut().for_each(|commit| {
        if commit.id == commit_id {
            commit.update_description(String::from(description));
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