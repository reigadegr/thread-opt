use super::common::execute_policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{check_some, get_thread_tids, UNNAME_TIDS},
    PolicyTasks,
};
use flume::Sender;
use hashbrown::HashSet;
use libc::pid_t;
use likely_stable::{likely, unlikely};
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

struct StartTask<'b, 'a: 'b> {
    high_usage_tids: Option<HashSet<pid_t>>,
    args: &'b mut StartArgs<'a>,
    tx: &'b Sender<Vec<pid_t>>,
    usage_top1: pid_t,
    usage_top2: pid_t,
    finish: bool,
}

impl<'b, 'a: 'b> StartTask<'b, 'a> {
    fn new(start_args: &'b mut StartArgs<'a>) -> Self {
        Self {
            high_usage_tids: Some(HashSet::new()),
            args: start_args,
            tx: &UNNAME_TIDS.0,
            usage_top1: 0,
            usage_top2: 0,
            finish: false,
        }
    }

    fn start_task(&mut self) {
        self.args.controller.init_game(true);
        loop {
            let pid = self.args.activity_utils.top_app_utils.get_pid();
            if unlikely(pid != self.args.pid) {
                self.args.controller.init_default();
                return;
            }

            let task_map = self.args.activity_utils.tid_utils.get_task_map(*pid);

            if likely(self.finish) {
                execute_policy(task_map, self.usage_top1, self.usage_top2);
                std::thread::sleep(Duration::from_millis(1000));
            } else {
                let unname_tids = get_thread_tids(task_map, b"Thread-");
                #[cfg(debug_assertions)]
                debug!("发送即将开始");
                self.tx.send(unname_tids).unwrap();
                #[cfg(debug_assertions)]
                debug!("发送已经完毕，喵等待一段时间计算");
                std::thread::sleep(Duration::from_millis(100));
                self.args.controller.update_max_usage_tid();

                check_some! {tid1, self.args.controller.first_max_tid(), "无法获取最大负载tid"};
                check_some! {tid2, self.args.controller.second_max_tid(), "无法获取第二负载tid"};

                if let Some(set) = self.high_usage_tids.as_mut() {
                    set.insert(tid1);
                    set.insert(tid2);

                    #[cfg(debug_assertions)]
                    debug!("负载第一高:{tid1}\n第二高:{tid2}");
                    if likely(set.len() < 3) {
                        execute_policy(task_map, tid1, tid2);
                        if unlikely((tid1 - tid2).abs() < 3) {
                            self.args.controller.init_default();
                            #[cfg(debug_assertions)]
                            debug!("检测到tid差异为小于3，可能是打开后台再进的，完成判断");
                            set.clear();
                            self.high_usage_tids = None;
                            self.usage_top1 = tid1;
                            self.usage_top2 = tid2;
                            self.finish = true;
                        }
                    } else {
                        if unlikely((tid1 - tid2).abs() > 1) {
                            #[cfg(debug_assertions)]
                            debug!("tid差异过大，重新计算");
                            set.clear();
                            set.insert(tid1);
                            set.insert(tid2);
                            continue;
                        }

                        self.args.controller.init_default();
                        #[cfg(debug_assertions)]
                        debug!("检测到集合长度大于2，可以结束了");
                        set.clear();
                        self.high_usage_tids = None;
                        self.usage_top1 = tid1;
                        self.usage_top2 = tid2;
                        self.finish = true;
                        #[cfg(debug_assertions)]
                        debug!(
                            "最终结果为:{0}\n第二高:{1}",
                            self.usage_top1, self.usage_top2
                        );
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}

pub fn start_task(args: &mut StartArgs<'_>) {
    StartTask::new(args).start_task();
}
