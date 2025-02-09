rm -rf output
rm -rf $(find ./target -name "*thread-opt*")
python3 ./make.py build --release --nightly || python3 ./make.py build --release
nohup cargo clippy >/dev/null 2>&1 &
