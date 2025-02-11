pub mod lolm;
pub mod macro_common;
pub mod ue5;
pub mod unnamed;

macro_rules! top1_policy {
    ($Top: ident, $Only6: ident, $Only7: ident, $Middle: ident, $Backend: ident,$CommPrefix:expr) => {
        use flume::Sender;
        use libc::pid_t;

        struct StartTask<'b, 'a: 'b> {
            policy:&'b Policy<'b>,
            args: &'b mut StartArgs<'a>,
            tx: &'b Sender<Vec<pid_t>>,
            usage_top1: pid_t,
            finish: bool,
        }

        impl<'b, 'a: 'b> StartTask<'b, 'a> {
            fn new(start_args: &'b mut StartArgs<'a>,policy:&'b Policy) -> Self {
                Self {
                policy,
                    args: start_args,
                    tx: &UNNAME_TIDS.0,
                    usage_top1: 0,
                    finish: false,
                }
            }

            fn after_usage_task(&mut self) {
                let task_map = self
                    .args
                    .activity_utils
                    .tid_utils
                    .get_task_map(self.args.pid);
                Policy::new(&self.policy).execute_policy(task_map, self.usage_top1);
                std::thread::sleep(Duration::from_millis(1000));
            }

            fn start_task(&mut self) {
                self.args.controller.init_game(true);
                loop {
                    let pid = self.args.activity_utils.top_app_utils.get_pid();
                    if unlikely(pid != self.args.pid) {
                        self.args.controller.init_default();
                        return;
                    }

                    let task_map = self.args.activity_utils.tid_utils.get_task_map(pid);
                    if likely(self.finish) {
                        self.after_usage_task();
                    } else {
                        let unname_tids = get_thread_tids(task_map, $CommPrefix);
                        #[cfg(debug_assertions)]
                        debug!("发送即将开始");
                        self.tx.send(unname_tids).unwrap();
                        #[cfg(debug_assertions)]
                        debug!("发送已经完毕");
                        std::thread::sleep(Duration::from_millis(100));
                        self.args.controller.update_max_usage_tid();
                        check_some! {tid1, self.args.controller.first_max_tid(), "无法获取最大负载tid"};
                        self.usage_top1 = tid1;
                        self.args.controller.init_default();
                        self.finish = true;
                        #[cfg(debug_assertions)]
                        debug!("计算后最终结果为:{0}\n", self.usage_top1);
                        continue;
                    }

                    std::thread::sleep(Duration::from_millis(1000));
                }
            }
        }

        pub fn start_task(args: &mut StartArgs<'_>) {
            let policy=Policy{
                top:&TOP,
               only6: &ONLY6,
              only7: &ONLY7,
              middle: &MIDDLE,
              background: &BACKEND,
        };
            StartTask::new(args,&policy).start_task();
        }

    };
}

use top1_policy;
