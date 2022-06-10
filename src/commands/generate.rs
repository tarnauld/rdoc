use crate::models::commit;
use crate::templates;
use crate::utils::{css, file, say};
use clap::Command;
use new_string_template::template::Template;
use std::collections::HashMap;

pub fn init_generate_command<'authors>() -> Command<'authors> {
    return Command::new("generate").about("Generate HTML report for the current GIT repository.");
}

pub fn generate() {
    let commits: Vec<commit::CommitInfo> = file::get_commits_from_index();
    create_index_html(&commits);
    css::generate_css_file();
    generate_files(commits);
    templates::fingerprint::save_favicon();
}

fn create_index_html(commits: &Vec<commit::CommitInfo>) {
    let templ = Template::new(templates::index::template_index());
    let data = {
        let mut map = HashMap::new();
        map.insert("header", String::from(templates::header::template_header()));
        map.insert("commits", generate_links(commits));
        map.insert("footer", String::from(templates::footer::template_footer()));
        map
    };

    let rendered = templ.render(&data).expect("Expected Result to be Ok");

    match file::save_file("index.html", &rendered) {
        Ok(_) => say::log("HTML report generated!"),
        Err(_) => println!("Cannot generate HTML report."),
    };
}

fn generate_links(commits: &Vec<commit::CommitInfo>) -> String {
    let mut links = String::new();

    commits.iter().for_each(|commit| {
        links.push_str(&*templates::link::template_link(&commit).as_str());
    });

    return links;
}

fn generate_files(commits: Vec<commit::CommitInfo>) {
    commits.iter().for_each(|commit| {
        let templ = Template::new(templates::commit::template_commit());

        let authors = templates::authors::template_authors(&commit.authors);
        let fingerprint = templates::fingerprint::template_fingerprint();
        let tags = templates::tags::template_tags(&commit.tags);

        let data = {
            let mut map = HashMap::new();
            map.insert("header", templates::header::template_header());
            map.insert("commit_id", &commit.id);
            map.insert("fingerprint", &*fingerprint.as_str());
            map.insert("commit_authors", &*authors.as_str());
            map.insert("commit_tags", &*tags.as_str());
            map.insert("commit_date", &commit.date);
            map.insert("commit_message", &commit.message);
            map.insert("commit_description", &commit.description);
            map.insert("footer", templates::footer::template_footer());
            map
        };

        let rendered = templ.render(&data).expect("Expected Result to be Ok");

        match file::save_file(format!("{}.html", commit.id).as_str(), &rendered) {
            Ok(_) => (),
            Err(_) => (),
        }
    })
}
