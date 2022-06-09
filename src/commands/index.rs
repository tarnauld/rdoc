use clap::Command;
use git2::{Repository, Revwalk};
use chrono::prelude::*;

pub fn init_index_command<'help>() -> Command<'help> {
    return Command::new("index").about("Generate the rdoc index.");
}

pub fn create_index() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let e = *get_revwalk(&repo);
    print_infos(&repo, e);
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

fn print_infos(repo: &Repository, revwalk: Revwalk) {
    for commit_id in revwalk {
        match commit_id {
            Ok(id) => get_commit(&repo, id),
            Err(e) => panic!("{}", e),
        }
    }
}

fn get_commit(repo: &Repository, commit_id: git2::Oid) {
    match repo.find_commit(commit_id) {
        Ok(commit) => {
            let commit_message = match commit.message() {
                Some(e) => e,
                None => "",
            };
            println!(
                "{{
                id: {},
                date: {:?},
                message: {},
                author: {}
             }}",
                commit.id(),
                NaiveDateTime::from_timestamp(commit.time().seconds(), 0),
                commit_message.replace("\n", ""),
                commit.author()
            )
        }
        Err(e) => panic!("{}", e),
    }
}