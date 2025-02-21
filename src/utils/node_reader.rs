use anyhow::Result;
use compact_str::CompactString;
use std::{fs::File, io::Read, path::Path};

pub fn read_file(file: &Path) -> Result<CompactString> {
    let s = std::fs::read_to_string(file)?;
    Ok(CompactString::new(s.trim()))
}

pub fn read_to_byte(file: &str) -> Result<[u8; 16]> {
    let mut file = File::open(file)?;
    let mut buffer = [0; 16];
    let _ = file.read(&mut buffer)?;
    Ok(buffer)
}
