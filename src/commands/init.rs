use clap::Command;
use ferris_says::say;
use std::fs;
use std::io::{stdout, BufWriter};

pub fn init_init_command<'help>() -> Command<'help> {
    return Command::new("init")
        .about("Create an empty Rdoc directory");
}

pub fn init_project() {
    let result = fs::create_dir("./.rdoc");
    let display = match result {
        Ok(_) => "rdoc is now installed. Welcome!",
        Err(_) => "rdoc is already installed. Nothing to do."
    };
    log(display.as_bytes());
}

fn log(s: &[u8]) {
    let width = 32;
    let mut writer = BufWriter::new(stdout());
    say(s, width, &mut writer).unwrap();
}
