use std::{fs::read_to_string, path::PathBuf};

use vizia::prelude::*;

#[derive(Lens, Data, Clone)]
pub struct File {
    pub path: PathBuf,
    pub data: String,
}

impl File {
    pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        let data = read_to_string(&path)?;
        Ok(Self { path, data })
    }
}
