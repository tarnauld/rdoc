mod commands;
mod matcher;
use clap::Command;
use commands::{index, init};

pub fn main() {
    let cmd = Command::new("rdoc")
        .bin_name("rdoc")
        .subcommand_required(true)
        .subcommand(init::init_init_command())
        .subcommand(index::init_index_command());

    matcher::match_commands(cmd.get_matches());
}
