use anyhow::Result;
use compact_str::CompactString;
use minivec::MiniVec;
use std::path::Path;

pub fn read_file(file: &Path) -> Result<CompactString> {
    let s = std::fs::read_to_string(file)?;
    Ok(CompactString::new(s.trim()))
}

pub fn read_to_byte(file: &Path) -> Result<MiniVec<u8>> {
    let vec: Vec<u8> = std::fs::read(file)?;
    let s: MiniVec<u8> = MiniVec::from(vec.as_slice());
    Ok(s)
}
