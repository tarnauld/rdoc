use crate::utils::say;
use clap::Command;
use std::fs;

pub fn init_init_command<'init>() -> Command<'init> {
    return Command::new("init").about("Create an empty Rdoc directory");
}

pub fn init_project() {
    let result = fs::create_dir("./.rdoc");
    let display = match result {
        Ok(_) => "rdoc is now installed. Welcome!",
        Err(_) => "rdoc is already installed. Nothing to do.",
    };
    say::log(display.as_bytes());
}
