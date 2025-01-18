mod shared;
use log::info;
use shared::get_top_app::TopAppUtils;
use shared::logger::init_log;
use std::fs;
use std::process;
use std::time::Duration;

fn init_misc() {
    let _ = init_log();
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/tasks", self_pid.to_string());
}

fn main() -> anyhow::Result<()> {
    init_misc();
    info!("Hello, world!");

    let mut binding = Looper::new();
    loop {
        let name = binding.get_top_app();

        info!("topappname: -{}-", name);
        std::thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}

struct Looper {
    windows_info: TopAppUtils,
}

impl Looper {
    fn new() -> Self {
        Self {
            windows_info: TopAppUtils::new(),
        }
    }
    fn get_top_app(&mut self) -> &str {
        self.windows_info.get_top_app()
    }
}
