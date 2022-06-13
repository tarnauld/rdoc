use crate::utils::gitmoji;
use clap::Command;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::{error::InquireError, Select};

pub fn init_gitmoji_command<'gitmoji>() -> Command<'gitmoji> {
    return Command::new("gitmoji").about("Lists emojis available (beta)");
}

pub fn list_emojis() {
    let options: Vec<String> = build_vec_emojis();

    let ans: Result<String, InquireError> =
        Select::new("Which gitmoji are you looking for?", options).prompt();

    match ans {
        Ok(choice) => {
            let emoji: Vec<&str> = choice.split(':').collect();
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(emoji[0].to_owned()).unwrap();
            println!("{} is now in your clipboard", emoji[0])
        }
        Err(_) => println!("There was an error, please try again"),
    }
}

fn build_vec_emojis() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let map = gitmoji::gitmoji_description();

    map.iter().for_each(|(key, value)| {
        vec.push(format!("{}: {}", key, value));
    });

    vec
}
