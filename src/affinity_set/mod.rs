pub mod policy;
use anyhow::Result;
// 定义 Executable trait
pub trait Executable {
    fn execute(&self);
}
