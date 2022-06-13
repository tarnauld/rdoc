use crate::utils::file;
use crate::utils::repo;
use clap::{Arg, Command};
use std::path::Path;

pub fn init_index_command<'index>() -> Command<'index> {
    return Command::new("index").about("Generate the rdoc index.").arg(
        Arg::new("force")
            .long("force")
            .short('f')
            .takes_value(false),
    );
}

pub fn create_index(force: bool) {
    let commits = repo::get_commits();

    if check_if_index_exists() && !force {
        println!(
            "The index file already exists. 
Try `rdoc update` if you want to update or `rdoc index --force` if you want to reset all changes.
        "
        );
        return;
    }

    match file::write_file(commits) {
        Ok(_) => (),
        Err(_) => println!("Rdoc is not installed. Please considering using `rdoc init` command"),
    };
}

fn check_if_index_exists() -> bool {
    Path::new("./.rdoc/index.json").exists()
}
