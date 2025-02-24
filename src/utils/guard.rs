use core::ptr::NonNull;
use libc::closedir;

pub struct DirGuard(Option<NonNull<libc::DIR>>);

impl DirGuard {
    /// 创建一个新的 `DirGuard`，包装一个 `DIR*` 指针。
    pub fn new(dir: *mut libc::DIR) -> Option<Self> {
        let ptr = NonNull::new(dir);
        ptr.map(|p| Self(Some(p)))
    }

    /// 关闭目录并释放资源。
    pub fn close(&mut self) {
        if let Some(dir) = self.0.take() {
            let _ = unsafe { closedir(dir.as_ptr()) };
        }
    }
}

impl Drop for DirGuard {
    fn drop(&mut self) {
        self.close();
    }
}
