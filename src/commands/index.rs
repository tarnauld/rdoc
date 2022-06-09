use crate::utils::file;
use crate::utils::repo;
use clap::Command;

pub fn init_index_command<'index>() -> Command<'index> {
    return Command::new("index").about("Generate the rdoc index.");
}

pub fn create_index() {
    let commits = repo::get_commits();

    match file::write_file(commits) {
        Ok(_) => (),
        Err(_) => println!("Rdoc is not installed. Please considering using `rdoc init` command"),
    };
}
