rm -rf output
rm -rf $(find ./target -name "*thread-opt*")
rm -rf $(find ./target -name "*thread_opt*")
cargo fmt
RUSTFLAGS="-C default-linker-libraries" python3 ./make.py build --release --nightly
