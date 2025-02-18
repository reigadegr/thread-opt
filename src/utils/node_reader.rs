use anyhow::Result;
use compact_str::CompactString;
use heapless::Vec;
use std::{fs::File, io::Read, path::Path};

pub fn read_file(file: &Path) -> Result<CompactString> {
    let s = std::fs::read_to_string(file)?;
    Ok(CompactString::new(s.trim()))
}

pub fn read_to_byte(file: &str) -> Result<Vec<u8, 16>> {
    let mut file = File::open(file)?;
    let mut temp_buffer = [0; 16];
    let _ = file.read(&mut temp_buffer)?;
    let buffer: Vec<u8, 16> = Vec::from_slice(&temp_buffer).unwrap();
    Ok(buffer)
}
