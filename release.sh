sed -i 's/edition = "2021"/edition = "2024"/g' Cargo.toml
cargo fmt
sed -i 's/edition = "2024"/edition = "2021"/g' Cargo.toml
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")
cargo fmt
RUSTFLAGS="-C default-linker-libraries" python3 ./make.py build --release --nightly
