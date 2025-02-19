use crate::utils::priority_utils::priority_initer::create_prio_obj;
use libc::sched_param;
use once_cell::sync::Lazy;

// 定义静态优先级的宏
macro_rules! priority_once_lazy {
    ($name:ident, $init:expr) => {
        pub static $name: Lazy<sched_param> = Lazy::new(|| create_prio_obj($init));
    };
}

// // 定义获取静态优先级值的宏
macro_rules! priority_getter {
    ($name:ident, $static_name:ident) => {
        pub fn $name() -> &'static sched_param {
            &$static_name
        }
    };
}

// 使用宏定义静态优先级值
priority_once_lazy!(TOP_PRIORITY, 10);
priority_once_lazy!(ONLY7_PRIORITY, 5);
priority_once_lazy!(ONLY6_PRIORITY, 5);
priority_once_lazy!(MIDDLE_PRIORITY, 20);
priority_once_lazy!(BACKGROUND_PRIORITY, 50);
priority_once_lazy!(MIDDLE_BACKGROUND_PRIORITY, 30);

// 使用宏定义获取优先级值的函数
priority_getter!(get_top_priority, TOP_PRIORITY);
priority_getter!(get_only7_priority, ONLY7_PRIORITY);
priority_getter!(get_only6_priority, ONLY6_PRIORITY);
priority_getter!(get_middle_priority, MIDDLE_PRIORITY);
priority_getter!(get_background_priority, BACKGROUND_PRIORITY);
priority_getter!(get_middle_background_priority, MIDDLE_BACKGROUND_PRIORITY);
