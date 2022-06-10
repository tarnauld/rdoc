mod commands;
mod matcher;
mod models;
mod utils;
mod templates;

pub fn main() {
    let cmd = commands::init_commands();
    matcher::match_commands(cmd.get_matches());
}
