mod init;
mod matcher;
use clap::Command;
use init::init_sub_command;

pub fn main() {
    let cmd = Command::new("rdoc")
        .bin_name("rdoc")
        .subcommand_required(true)
        .subcommand(init_sub_command());

    matcher::match_commands(cmd.get_matches());
}
