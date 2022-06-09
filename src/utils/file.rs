use crate::models::commit;
use std::fs::File;
use std::io::prelude::*;

pub fn write_file(commits_info: Vec<commit::CommitInfo>) -> std::io::Result<()> {
    let mut file = File::create("./.rdoc/index.json")?;
    let json_commits_infos = serde_json::to_string_pretty(&commits_info)?;
    file.write_all(json_commits_infos.as_bytes())?;
    Ok(())
}

fn parse_index_file() -> std::io::Result<Vec<commit::CommitInfo>> {
    let mut file = File::open("./.rdoc/index.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let commits: Vec<commit::CommitInfo> = serde_json::from_str(&contents).unwrap();
    return Ok(commits);
}

pub fn get_commits_from_index() -> Vec<commit::CommitInfo> {
    return match parse_index_file() {
        Ok(e) => e,
        Err(_) => {
            println!("Report index is missing. Did you use `rdoc index` command?");
            return Vec::from([]);
        }
    };
}

pub fn save_file(source: &str, doc: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("./.rdoc/{}", source))?;
    file.write_all(doc.as_bytes())?;
    Ok(())
}
