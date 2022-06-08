use clap::Command;
use ferris_says::say;
use std::fs;
use std::io::{stdout, BufWriter};

pub fn init_sub_command<'help>() -> Command<'help> {
    return Command::new("init")
        .about("Create an empty Rdoc directory");
}

pub fn init_project() {
    let result = fs::create_dir("./.rdoc");
    match result {
        Ok(_) => {
            log(b"rdoc is now installed. Welcome!");
        }
        Err(_) => {
            log(b"rdoc is already installed. Nothing to do.");
        }
    }
}

fn log(s: &[u8]) {
    let width = 32;
    let mut writer = BufWriter::new(stdout());
    say(s, width, &mut writer).unwrap();
}
