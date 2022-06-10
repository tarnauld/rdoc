use crate::models::commit;
use chrono::prelude::*;
use git2::{Repository, Revwalk};

fn open_repository(path: &str) -> Repository {
    match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

fn get_revwalk<'scope>(repo: &'scope Repository) -> Box<Revwalk<'scope>> {
    return match repo.revwalk() {
        Ok(revwalk) => Box::new(push_head(revwalk)),
        Err(e) => panic!("{}", e),
    };
}

pub fn get_commits() -> Vec<commit::CommitInfo> {
    let repo = open_repository("./");
    let revwalk = *get_revwalk(&repo);
    let mut array = Vec::new();
    for commit_id in revwalk {
        match commit_id {
            Ok(id) => array.push(find_commit(&repo, id)),
            Err(e) => panic!("{}", e),
        }
    }
    return array;
}

fn push_head(mut revwalk: Revwalk) -> Revwalk {
    match revwalk.push_head() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    return revwalk;
}

fn find_commit(repo: &Repository, commit_id: git2::Oid) -> commit::CommitInfo {
    match repo.find_commit(commit_id) {
        Ok(commit) => {
            return commit::CommitInfo {
                id: format!("{}", commit.id()),
                author: extract_commit_author(&commit),
                date: format!(
                    "{}",
                    NaiveDateTime::from_timestamp(commit.time().seconds(), 0)
                ),
                message: extract_commit_message(&commit),
                tag: String::from(""),
                authors: extract_commit_author(&commit)
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
