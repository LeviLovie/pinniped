use anyhow::{Context, Result};
use log::info;

pub struct File {
    pub name: String,
    pub path: String,
    pub absolute_path: String,
    pub contents: String,
}

impl File {
    pub fn new(name: String, path: String) -> Result<Self> {
        let absolute_path = match std::fs::canonicalize(&path) {
            Ok(path) => path.to_str().unwrap().to_string(),
            Err(e) => {
                return Err(e).context(format!("Error getting absolute path: {}", path))?;
            }
        };

        Ok(Self {
            name,
            path,
            absolute_path,
            contents: String::new(),
        })
    }

    pub fn read(&mut self) -> Result<()> {
        self.contents = match std::fs::read_to_string(&self.absolute_path) {
            Ok(contents) => contents,
            Err(e) => {
                return Err(e).context(format!("Error reading file: {}", self.absolute_path))?;
            }
        };
        info!(
            "Read file: {} ({} KB)",
            self.absolute_path,
            self.contents.len() as f32 / 1024.0
        );
        Ok(())
    }
}
