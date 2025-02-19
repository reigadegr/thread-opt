rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")
cargo fmt
RUSTFLAGS="-C default-linker-libraries" python3 ./make.py build --release --nightly
