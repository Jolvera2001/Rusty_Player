use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileList {
    pub files: Vec<File>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub last_modified: String,
}