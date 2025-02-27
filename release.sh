cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")
RUSTFLAGS="-C default-linker-libraries" python3 ./make.py build --release --nightly
