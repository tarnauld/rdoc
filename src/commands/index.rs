use chrono::prelude::*;
use clap::Command;
use git2::{Repository, Revwalk};
use std::fs::File;
use std::io::prelude::*;
use serde_derive::{Serialize, Deserialize};

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

    match write_file(infos) {
        Ok(_) => (),
        Err(_) => panic!(),
    };
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
                id: format!("{}", commit.id()),
                author: extract_commit_author(&commit),
                date: format!("{}", NaiveDateTime::from_timestamp(commit.time().seconds(), 0)),
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

fn write_file(commits_info: Vec<CommitInfo>) -> std::io::Result<()> {
    let mut file = File::create("./.rdoc/index.json")?;
    let json_commits_infos = serde_json::to_string_pretty(&commits_info)?;
    file.write_all(json_commits_infos.as_bytes())?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct CommitInfo {
    id: String,
    author: String,
    date: String,
    message: String,
}