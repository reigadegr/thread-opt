use anyhow::Result;
use std::{fs, path::Path};
pub fn read_file(file: &Path) -> Result<String> {
    let s = fs::read_to_string(file)?;
    Ok(s.trim().to_string())
}
