mod activity;
mod looper;
mod misc;
use log::info;
use looper::Looper;
use misc::logger::init_misc;

fn main() -> anyhow::Result<()> {
    init_misc();
    info!("Hello, world!");
    Looper::new().enter_loop();
    Ok(())
}
