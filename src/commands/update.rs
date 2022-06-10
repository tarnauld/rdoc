use clap::{Command};

pub fn init_update_command<'index>() -> Command<'index> {
    return Command::new("update")
        .about("Update the rdoc index.");
}

pub fn update_index() {
    println!("Not implementated yet");
}