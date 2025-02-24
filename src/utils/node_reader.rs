use anyhow::{Result, anyhow};
use compact_str::CompactString;
use likely_stable::unlikely;
use std::{fs::File, io::Read};
use stringzilla::sz;
extern crate alloc;
use alloc::ffi::CString;

pub fn read_file(file: &str) -> Result<CompactString> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 32];
    let _ = file.read(&mut buffer)?;
    let pos = sz::find(buffer, b"\n");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub fn read_to_byte(file: &str) -> Result<[u8; 16]> {
    #[cfg(debug_assertions)]
    let start = minstant::Instant::now();
    let c_file = CString::new(file)?;
    let fd = unsafe { libc::open(c_file.as_ptr(), libc::O_RDONLY) };
    if unlikely(fd == -1) {
        return Err(anyhow!("Cannot open file."));
    }

    let mut buffer = [0u8; 16];
    let bytes_read =
        unsafe { libc::read(fd, buffer.as_mut_ptr().cast::<libc::c_void>(), buffer.len()) };
    unsafe {
        libc::close(fd);
    }

    if unlikely(bytes_read == -1) {
        return Err(anyhow!("Cannot read file."));
    }
    #[cfg(debug_assertions)]
    {
        let end = start.elapsed();
        log::debug!("comm时间: {:?}", end);
    }
    Ok(buffer)
}
