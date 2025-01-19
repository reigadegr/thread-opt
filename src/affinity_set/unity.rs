use super::Executable;
use anyhow::Result;
#[derive(Debug)]
enum CmdType {
    All,
    Top,
    Middle,
    Backend,
}

// 定义一个app结构体，包含一个CmdType枚举和一个tid字段
#[derive(Debug)]
pub struct App {
    cmd_type: CmdType,
    tid: i32,
}

// 为app结构体实现ExecuteCommand trait
impl Executable for App {
    fn execute(&self) -> () {
        match self.cmd_type {
            CmdType::All => {}
            CmdType::Top => {}
            CmdType::Middle => {}
            CmdType::Backend => {}
        }
    }
}

// 根据字符串和tid创建app结构体的函数
pub fn from_str(comm: &str, tid: i32) -> App {
    match comm {
        "UnityMain" => App {
            cmd_type: CmdType::Top,
            tid,
        },
        "UnityGfxDeviceW" | "UnityMultiRende" | "NativeThread" => App {
            cmd_type: CmdType::Middle,
            tid,
        },
        _ => App {
            cmd_type: CmdType::All,
            tid,
        },
    }
}
