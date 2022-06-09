use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitInfo {
    pub id: String,
    pub author: String,
    pub date: String,
    pub message: String,
    pub tag: String
}

impl CommitInfo {
    pub fn update_tag(&mut self, tags: String) {
        self.tag = tags;
    }
}