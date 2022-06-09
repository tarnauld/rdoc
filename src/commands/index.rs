use clap::Command;
use git2::{Repository, Revwalk};

pub fn init_index_command<'help>() -> Command<'help> {
    return Command::new("index").about("Generate the rdoc index.");
}

pub fn create_index() {
    let repo = match Repository::open("/Users/timotheearnauld/Documents/rust/rdoc") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut revwalk = match repo.revwalk() {
        Ok(revwallk) => revwallk,
        Err(e) => panic!("{}", e),
    };

    match revwalk.push_head() {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }

    print_ids(revwalk);
}

fn print_ids(revwalk: Revwalk) {
    for id in revwalk {
        match id {
            Ok(id) => println!("{}", id),
            Err(e) => panic!("{}", e),
        }
    }
}
