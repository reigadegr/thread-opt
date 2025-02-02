cargo fmt
cargo clippy || exit 0
rm -rf output
rm -rf $(find ./target -name "*thread-opt*")
python3 ./make.py build --debug --nightly >/dev/null 2>&1
