use anyhow::Result;
use compact_str::CompactString;
use std::{fs::File, io::Read};
use stringzilla::sz;

pub fn read_file(file: &str) -> Result<CompactString> {
    // let mut file = File::open(file)?;
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 32];
    let _ = file.read(&mut buffer)?;
    let pos = sz::find(buffer, b"\n");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub fn read_to_byte(file: &str) -> Result<[u8; 16]> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 16];
    let _ = file.read(&mut buffer)?;
    Ok(buffer)
}
