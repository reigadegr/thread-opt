use super::guard::FileGuard;
use anyhow::{Result, anyhow};
use compact_str::CompactString;
use core::ptr::copy_nonoverlapping;
use itoa::Buffer;
use libc::{O_RDONLY, O_WRONLY, c_void, open, pid_t, read, write};
use likely_stable::unlikely;
use stringzilla::sz;

pub fn read_file<const N: usize>(file: &[u8]) -> Result<CompactString> {
    let buffer = read_to_byte::<N>(file)?;
    let pos = sz::find(buffer, b"\n");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub fn read_to_byte<const N: usize>(file: &[u8]) -> Result<[u8; N]> {
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

pub fn write_to_byte<const N: usize>(file: &[u8], msg: &[u8]) -> Result<()> {
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

pub fn get_proc_path<const N: usize, const L: usize>(id: pid_t, file: &[u8]) -> [u8; N] {
    let mut buffer = [0u8; N];
    buffer[0..6].copy_from_slice(b"/proc/");

    let mut itoa_buf = Buffer::new();
    let id = itoa_buf.format(id).as_bytes();

    let id_length = id.len();

    unsafe {
        copy_nonoverlapping(id.as_ptr(), buffer.as_mut_ptr().add(6), id_length);
        copy_nonoverlapping(file.as_ptr(), buffer.as_mut_ptr().add(6 + id_length), L);
    }

    buffer
}
