use anyhow::Result;
use compact_str::CompactString;
use std::{fs::File, io::Read, path::Path};

pub fn read_file(file: &Path) -> Result<CompactString> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 128];
    let _ = file.read(&mut buffer)?;
    let null_pos = buffer.iter().position(|&b| b == 0x00);
    let buffer = null_pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub fn read_to_byte(file: &str) -> Result<[u8; 16]> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 16];
    let _ = file.read(&mut buffer)?;
    Ok(buffer)
}
