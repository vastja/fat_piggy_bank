use std::fs;
use std::io::Error;
use std::path::Path;

pub trait Importer {
    fn load(&self, uri: &str) -> Result<String, Error>;
}

pub struct FileImporter {}

impl Importer for FileImporter {
    fn load(&self, path: &str) -> Result<String, Error> {
        fs::read_to_string(Path::new(path))
    }
}
