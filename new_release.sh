rm -rf $(find ./target -name "*release*")
sh release.sh
nohup cargo clippy >/dev/null 2>&1 &
