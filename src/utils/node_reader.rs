use anyhow::Result;
use compact_str::CompactString;
use std::path::Path;
pub fn read_file(file: &Path) -> Result<CompactString> {
    let s = std::fs::read_to_string(file)?;
    Ok(CompactString::new(s.trim()))
}
