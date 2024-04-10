use crate::file_system::{ FileList, File };
use std::fs;

#[tauri::command]
pub fn get_files(path: String) -> Result<FileList, String> {
    let mut files: Vec<File> = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();
        let file = File {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.to_str().unwrap().to_string(),
            size: metadata.len(),
            last_modified: format!("{}", metadata.modified().unwrap()), //TODO: format date
        };
        files.push(file);
    }
    Ok(FileList { files })
}