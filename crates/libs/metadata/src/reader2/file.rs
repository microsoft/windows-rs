use super::*;

#[derive(Default)]
pub struct File {
    name: String,
    bytes: Vec<u8>,
    strings: usize,
    blobs: usize,
    tables: usize,
}

impl File {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let path = std::path::Path::new(path);
        let bytes = std::fs::read(&path)?;
        // Since the file was read successfully, we just assume it has a valid file name.
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        Ok(File {
            bytes,
            name,
            ..Default::default()
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
