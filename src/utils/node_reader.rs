use super::guard::FileGuard;
use anyhow::{Result, anyhow};
use compact_str::CompactString;
use core::ffi::CStr;
use libc::{O_RDONLY, O_WRONLY, c_void, open, read, write};
use likely_stable::unlikely;
use stringzilla::sz;
extern crate alloc;
use alloc::ffi::CString;

pub fn read_file<const N: usize>(file: &str) -> Result<CompactString> {
    let buffer = read_to_byte::<N>(file)?;
    let pos = sz::find(buffer, b"\n");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub fn read_to_byte<const N: usize>(file: &str) -> Result<[u8; N]> {
    let c_file = CString::new(file)?;
    let mut buffer = [0u8; N];
    unsafe {
        let fd = open(c_file.as_ptr(), O_RDONLY);
        if unlikely(fd == -1) {
            return Err(anyhow!("Cannot open file."));
        }
        let _fd_guard = FileGuard::new(fd);
        let bytes_read = read(fd, buffer.as_mut_ptr().cast::<c_void>(), N);

        if unlikely(bytes_read == -1) {
            return Err(anyhow!("Cannot read file."));
        }
    }
    Ok(buffer)
}

pub fn read_to_byte_sp<const N: usize>(file: &[u8]) -> Result<[u8; N]> {
    let mut buffer = [0u8; N];
    unsafe {
        let fd = open(file.as_ptr(), O_RDONLY);
        if unlikely(fd == -1) {
            return Err(anyhow!("Cannot open file."));
        }
        let _fd_guard = FileGuard::new(fd);
        let bytes_read = read(fd, buffer.as_mut_ptr().cast::<c_void>(), N);

        if unlikely(bytes_read == -1) {
            return Err(anyhow!("Cannot read file."));
        }
    }
    Ok(buffer)
}

pub fn write_to_byte<const N: usize>(file: &CStr, msg: &str) -> Result<()> {
    let msg = CString::new(msg)?;
    unsafe {
        let fd = open(file.as_ptr(), O_WRONLY);
        if unlikely(fd == -1) {
            return Err(anyhow!("Cannot open file."));
        }
        let _fd_guard = FileGuard::new(fd);
        let bytes_write = write(fd, msg.as_ptr().cast::<c_void>(), N);

        if unlikely(bytes_write == -1) {
            return Err(anyhow!("Cannot write file."));
        }
    }
    Ok(())
}
