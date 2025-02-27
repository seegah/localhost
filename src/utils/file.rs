use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    fs::read(path).map_err(Into::into)
}

pub fn write_file(path: &Path, content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, content).map_err(Into::into)
}