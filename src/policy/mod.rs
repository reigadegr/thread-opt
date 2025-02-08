pub mod name_match;
pub mod pkg_cfg;
pub mod usage;
use pkg_cfg::StartArgs;

trait PolicyTasks {
    fn start_task(args: &mut StartArgs);
}
