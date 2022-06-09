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

    let mut e = *get_revwalk(&repo);
    let _x = e.push_head();
    print_ids(e);
}

fn get_revwalk<'scope>(repo: &'scope Repository) -> Box<Revwalk<'scope>> {
    return match repo.revwalk() {
        Ok(revwalk) => {
            Box::new(revwalk)
        },
        Err(e) => panic!("{}", e),
    };
}

fn print_ids(revwalk: Revwalk) {
    for id in revwalk {
        match id {
            Ok(id) => println!("{}", id),
            Err(e) => panic!("{}", e),
        }
    }
}
