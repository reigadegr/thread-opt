use anyhow::Result;
use compact_str::CompactString;
use std::{fs, path::Path};
pub fn read_file(file: &Path) -> Result<CompactString> {
    let s = fs::read_to_string(file)?;
    Ok(CompactString::new(s.trim()))
}
