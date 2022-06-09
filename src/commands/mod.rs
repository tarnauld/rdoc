pub mod index;
pub mod init;
pub mod tag;
pub mod authors;
pub mod generate;
use clap::Command;

pub fn init_commands<'commands>() -> Command<'commands> {
    return Command::new("rdoc")
        .bin_name("rdoc")
        .subcommand_required(true)
        .subcommand(init::init_init_command())
        .subcommand(index::init_index_command())
        .subcommand(tag::init_tag_command())
        .subcommand(authors::init_authors_command())
        .subcommand(generate::init_generate_command());
}