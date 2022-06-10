use crate::models::commit;
use crate::utils::{file, repo, say};
use clap::Command;

pub fn init_update_command<'index>() -> Command<'index> {
    return Command::new("update").about("Update the rdoc index.");
}

pub fn update_index() {
    let git_commits = repo::get_commits();
    let mut index_commits = file::get_commits_from_index();

    let merge = merge_commits(git_commits, &mut index_commits);

    match file::write_file(*merge) {
         Ok(_) => say::log("Index successfully updated!"),
         Err(_) => println!("An error occured while updating the index"),
    };
}

fn merge_commits(
    git_commits: Vec<commit::CommitInfo>,
    index_commits: &mut Vec<commit::CommitInfo>,
) -> Box<Vec<commit::CommitInfo>> {
    let mut result = Vec::new();

    git_commits.iter().for_each(|git_commit| {
        if !index_commits
            .iter()
            .any(|commit| commit.id == git_commit.id)
        {
            println!("Add commit {} to index file.", git_commit.id);
            result.push(commit::CommitInfo {
                id: String::from(&*git_commit.id),
                author: String::from(&*git_commit.author),
                authors: String::from(&*git_commit.authors),
                date: String::from(&*git_commit.date),
                description: String::from(&*git_commit.description),
                message: String::from(&*git_commit.message),
                tags: String::from(&*git_commit.tags),
            });
        };
    });
    result.append(index_commits);
    Box::new(result)
}
