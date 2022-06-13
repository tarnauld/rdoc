pub mod authors;
pub mod describe;
pub mod generate;
pub mod index;
pub mod init;
pub mod search;
pub mod show;
pub mod tag;
pub mod update;
use clap::Command;

pub fn init_commands<'commands>() -> Command<'commands> {
    return Command::new("rdoc")
        .bin_name("rdoc")
        .subcommand_required(true)
        .subcommand(init::init_init_command())
        .subcommand(index::init_index_command())
        .subcommand(tag::init_tag_command())
        .subcommand(authors::init_authors_command())
        .subcommand(generate::init_generate_command())
        .subcommand(describe::init_describe_command())
        .subcommand(update::init_update_command())
        .subcommand(search::init_search_command())
        .subcommand(show::init_show_command());
}
