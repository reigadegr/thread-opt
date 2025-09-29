use super::guard::FileGuard;
use anyhow::{Result, anyhow};
use compact_str::CompactString;
use core::ptr::copy_nonoverlapping;
use itoa::Buffer;
use libc::{O_CREAT, O_TRUNC, O_WRONLY, c_void, chmod, chown, open, pid_t, write};
use likely_stable::unlikely;
use std::{
    fs::File,
    io::{ErrorKind, Read},
    str::from_utf8,
};
use stringzilla::sz;

pub fn read_file<const N: usize>(file: &[u8]) -> Result<CompactString> {
    let buffer = read_to_byte::<N>(file)?;
    let pos = sz::find(buffer, b"\0");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub fn read_to_byte<const N: usize>(file: &[u8]) -> Result<[u8; N]> {
    let end = sz::find(file, b"\0").unwrap_or(N);
    let file = &file[..end];
    let file = from_utf8(file)?;

    let Ok(mut file) = File::open(file) else {
        return Err(anyhow!("Cannot open file."));
    };
    let mut buffer = [0u8; N];
    let mut total_read = 0;
    loop {
        let n = match file.read(&mut buffer[total_read..]) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.into()),
        };
        total_read += n;
    }

    Ok(buffer)
}

pub fn write_to_byte(file: &[u8], msg: &[u8]) -> Result<()> {
    unsafe {
        let fd = open(file.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC, 0o666);
        if unlikely(fd == -1) {
            return Err(anyhow!("Cannot open file."));
        }
        let _fd_guard = FileGuard::new(fd);
        let bytes_write = write(fd, msg.as_ptr().cast::<c_void>(), msg.len());

        if unlikely(bytes_write == -1) {
            return Err(anyhow!("Cannot write file."));
        }
    }
    Ok(())
}

pub fn lock_val(file: &[u8], msg: &[u8]) -> Result<()> {
    unsafe {
        chown(file.as_ptr(), 0, 0);
        chmod(file.as_ptr(), 0o644);
        if write_to_byte(file, msg).is_err() {
            return Err(anyhow!("Cannot write file."));
        }
        chmod(file.as_ptr(), 0o444);
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
