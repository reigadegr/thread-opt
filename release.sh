rm -rf output
rm -rf $(find ./target -name "*thread-opt*")
rm -rf $(find ./target -name "*thread_opt*")
cargo fmt
python3 ./make.py build --release --nightly
