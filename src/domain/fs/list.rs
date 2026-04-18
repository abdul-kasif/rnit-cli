#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
}

pub fn list_current_dir() -> Result<Vec<FileEntry>, std::io::Error> {
    Ok(vec![
        FileEntry {
            name: "example.txt".to_string(),
        },
        FileEntry {
            name: "src".to_string(),
        },
    ])
}
