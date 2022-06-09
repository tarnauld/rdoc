use clap::Arg;
use clap::Command;
use crate::utils::file;
use crate::models::commit;

pub fn init_tag_command<'tag>() -> Command<'tag> {
    return Command::new("tag")
        .about("Add tag for a commit")
        .arg(Arg::new("commit").required(true))
        .arg(Arg::new("tag").required(true));
}

pub fn create_tag(commit: Option<&str>, tag: Option<&str>) {
    let commit_id = get_option(commit);
    let tag = get_option(tag);

    let mut commits: Vec<commit::CommitInfo> = file::get_commits_from_index();
    commits.iter_mut().for_each(|commit| {
        if commit.id == commit_id {
            commit.update_tag(String::from(tag));
            println!("{}:{}", commit.id, commit.message);
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