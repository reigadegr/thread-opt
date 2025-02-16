extern crate alloc;

use alloc::ffi::CString;
use core::ffi::CStr;
use libc::{DIR, closedir, opendir, readdir};
use no_std_io::io;

pub fn read_dir(path: &str) -> io::Result<()> {
    let c_path = CString::new(path).unwrap();
    let dir_ptr = unsafe { opendir(c_path.as_ptr()) };

    if dir_ptr.is_null() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open directory",
        ));
    }

    loop {
        let entry_ptr = unsafe { readdir(dir_ptr) };
        if entry_ptr.is_null() {
            break;
        }

        let entry_name = unsafe { CStr::from_ptr((*entry_ptr).d_name.as_ptr()) };
    }

    unsafe { closedir(dir_ptr) };
    Ok(())
}
