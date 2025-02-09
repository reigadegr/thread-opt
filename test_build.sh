cargo fmt
rm -rf output
rm -rf $(find ./target -name "*thread-opt*")
python3 ./make.py build --debug --nightly
nohup cargo clippy >/dev/null 2>&1 &
