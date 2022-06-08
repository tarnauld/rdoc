use clap::Command;

pub fn init_index_command<'help>() -> Command<'help> {
    return Command::new("index")
        .about("Generate the rdoc index.");
}

pub fn create_index() {
    println!("Generate index...");
}