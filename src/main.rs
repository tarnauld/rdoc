mod init;
mod matcher;
mod index;
use clap::Command;
use init::init_init_command;
use index::init_index_command;

pub fn main() {
    let cmd = Command::new("rdoc")
        .bin_name("rdoc")
        .subcommand_required(true)
        .subcommand(init_init_command())
        .subcommand(init_index_command());

    matcher::match_commands(cmd.get_matches());
}
