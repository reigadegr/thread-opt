use anyhow::{Result, anyhow};
use compact_str::CompactString;
use core::ptr::copy_nonoverlapping;
use itoa::Buffer;
use libc::chmod;
use std::{
    // fs::File,
    io::{ErrorKind, Read},
    str::from_utf8,
};
use stringzilla::sz;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

pub async fn lock_value(path: &[u8], value: &[u8]) {
    unsafe {
        let _ = chmod(path.as_ptr(), 0o666);
        let _ = write_to_byte(path, value).await;
        let _ = chmod(path.as_ptr(), 0o444);
    }
}

pub async fn read_file<const N: usize>(file: &[u8]) -> Result<CompactString> {
    let buffer = read_to_byte::<N>(file).await?;
    let pos = sz::find(buffer, b"\0");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub async fn read_to_byte<const N: usize>(file: &[u8]) -> Result<[u8; N]> {
    let end = sz::find(file, b"\0").unwrap_or(N);
    let file = &file[..end];
    let file = from_utf8(file)?;

    let mut file = File::open(file)
        .await
        .map_err(|e| anyhow!("Cannot open file: {e}"))?;

    let mut buffer = [0u8; N];

    match file.read_exact(&mut buffer).await {
        Ok(_) => Ok(buffer),
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok(buffer),
        Err(e) => Err(e.into()),
    }
}

pub async fn write_to_byte(file: &[u8], msg: &[u8]) -> Result<()> {
    let end = sz::find(file, b"\0").unwrap_or(file.len());
    let file = &file[..end];
    let file = from_utf8(file)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file)
        .await?;

    let _ = file.write_all(msg).await;
    Ok(())
}

pub fn get_proc_path<const N: usize>(id: i32, file: &[u8]) -> [u8; N] {
    let mut buffer = [0u8; N];
    buffer[0..6].copy_from_slice(b"/proc/");

    let mut itoa_buf = Buffer::new();
    let id = itoa_buf.format(id).as_bytes();

    let id_length = id.len();

    unsafe {
        copy_nonoverlapping(id.as_ptr(), buffer.as_mut_ptr().add(6), id_length);
        copy_nonoverlapping(
            file.as_ptr(),
            buffer.as_mut_ptr().add(6 + id_length),
            file.len(),
        );
    }
    buffer
}
