cargo fmt
cargo clippy || exit 0
rm -rf output
nohup python3 ./make.py build --debug --nightly >/dev/null 2>&1
