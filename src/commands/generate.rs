use crate::models::commit;
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
}

fn create_index_html(commits: &Vec<commit::CommitInfo>) {
    let doc = "<html>
            <head>
                <title>Rdoc - HTML Report</title>
                <meta name=\"Metadata::Author\" content=\"Rdoc\"/>
                <link rel=\"stylesheet\" href=\"./style.css\"/>
            </head>
            <body>
                <div class=\"header\">
                    <span>Rdoc HTML Report</span>
                </div>
                <ul>
                {commits}
                </ul>
            </body>
        </html>";
    let templ = Template::new(doc);
    let data = {
        let mut map = HashMap::new();
        map.insert("commits", generate_links(commits));
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
        let s: String = format!(
            "<li><a href=\"{}.html\">{}</a>: {}</li>",
            commit.id,
            &commit.id[..6],
            commit.message
        );
        links.push_str(s.as_str());
    });

    return links;
}

fn generate_files(commits: Vec<commit::CommitInfo>) {
    commits.iter().for_each(|commit| {
        let doc = "<html>
        <head>
            <title>Rdoc - HTML Report</title>
            <meta name=\"Metadata::Author\" content=\"Rdoc\"/>
            <link rel=\"stylesheet\" href=\"./style.css\"/>
        </head>
        <body>
            <div class=\"header\">
                <span>Rdoc HTML Report</span>
            </div>
            <h1>{commit_id}</h1>
            <h2>{commit_authors}</h2>
            <h3>{commit_date}</h3>
            <p>
            {commit_message}
            </p>
        </body>
    </html>";
        let templ = Template::new(doc);

        let data = {
            let mut map = HashMap::new();
            map.insert("commit_id", &commit.id);
            map.insert("commit_authors", &commit.authors);
            map.insert("commit_date", &commit.date);
            map.insert("commit_message", &commit.message);
            map
        };

        let rendered = templ.render(&data).expect("Expected Result to be Ok");

        match file::save_file(format!("{}.html", commit.id).as_str(), &rendered) {
            Ok(_) => (),
            Err(_) => (),
        }
    })
}
