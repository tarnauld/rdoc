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

    let e = *get_revwalk(&repo);
    print_ids(e);
}

fn get_revwalk<'scope>(repo: &'scope Repository) -> Box<Revwalk<'scope>> {
    return match repo.revwalk() {
        Ok(revwalk) => {
            Box::new(push_head(revwalk))
        },
        Err(e) => panic!("{}", e),
    };
}

fn push_head(mut revwalk: Revwalk) -> Revwalk {
    match revwalk.push_head() {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    } 
    return revwalk;
}

fn print_ids(revwalk: Revwalk) {
    for id in revwalk {
        match id {
            Ok(id) => println!("{}", id),
            Err(e) => panic!("{}", e),
        }
    }
}
