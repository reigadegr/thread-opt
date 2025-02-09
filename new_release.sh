cargo clean
sh release.sh
nohup cargo clippy >/dev/null 2>&1 &
