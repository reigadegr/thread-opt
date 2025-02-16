use anyhow::Result;
use compact_str::CompactString;
use std::path::Path;
pub fn read_file(file: &Path) -> Result<CompactString> {
    let s = std::fs::read_to_string(file)?;
    Ok(CompactString::new(s.trim()))
}

pub fn read_to_byte(file: &Path) -> Result<Vec<u8>> {
    let s: Vec<u8> = std::fs::read(file)?;
    // s.pop();
    Ok(s)
}
