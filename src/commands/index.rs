use chrono::prelude::*;
use clap::Command;
use git2::{Repository, Revwalk};

pub fn init_index_command<'help>() -> Command<'help> {
    return Command::new("index").about("Generate the rdoc index.");
}

pub fn create_index() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let revwalk = *get_revwalk(&repo);
    let infos = get_commits(&repo, revwalk);

    print_infos(infos);
}

fn get_revwalk<'scope>(repo: &'scope Repository) -> Box<Revwalk<'scope>> {
    return match repo.revwalk() {
        Ok(revwalk) => Box::new(push_head(revwalk)),
        Err(e) => panic!("{}", e),
    };
}

fn push_head(mut revwalk: Revwalk) -> Revwalk {
    match revwalk.push_head() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    return revwalk;
}

fn get_commits(repo: &Repository, revwalk: Revwalk) -> Vec<CommitInfo> {
    let mut array = Vec::new();
    for commit_id in revwalk {
        match commit_id {
            Ok(id) => array.push(find_commit(&repo, id)),
            Err(e) => panic!("{}", e),
        }
    }

    return array;
}

fn find_commit(repo: &Repository, commit_id: git2::Oid) -> CommitInfo {
    match repo.find_commit(commit_id) {
        Ok(commit) => {
            return CommitInfo {
                id: commit.id(),
                author: extract_commit_author(&commit),
                date: NaiveDateTime::from_timestamp(commit.time().seconds(), 0),
                message: extract_commit_message(&commit),
            };
        }
        Err(e) => panic!("{}", e),
    }
}

fn extract_commit_author(commit: &git2::Commit) -> String {
    return match commit.author().name() {
        Some(e) => String::from(e),
        None => String::from(""),
    };
}

fn extract_commit_message(commit: &git2::Commit) -> String {
    return match commit.message() {
        Some(e) => e.replace("\n", ""),
        None => String::from(""),
    };
}

fn print_infos(commit_infos: Vec<CommitInfo>) {
    for commit_info in commit_infos {
        println!(
            "{{
            id: {},
            author: {},
            date: {},
            message: {}
        }}",
            commit_info.id, commit_info.author, commit_info.date, commit_info.message
        );
    }
}

struct CommitInfo {
    id: git2::Oid,
    author: String,
    date: NaiveDateTime,
    message: String,
}
