use markdown;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitInfo {
    pub id: String,
    pub author: String,
    pub date: String,
    pub message: String,
    pub tags: String,
    pub authors: String,
    pub description: String,
}

impl CommitInfo {
    pub fn update_tag(&mut self, tags: String) {
        self.tags = String::from(format!("{};{}", self.tags, tags)).replace("none;", "");
    }

    pub fn update_authors(&mut self, authors: String) {
        self.authors = String::from(format!("{};{}", self.authors, authors));
    }

    pub fn update_description(&mut self, description: &String) {
        self.description = markdown::to_html(description.as_str());
    }
}
