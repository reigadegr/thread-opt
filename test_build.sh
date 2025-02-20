sed -i 's/edition = "2021"/edition = "2024"/g' Cargo.toml
cargo fmt
sed -i 's/edition = "2024"/edition = "2021"/g' Cargo.toml
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/debug -name "*thread-opt*")
python3 ./make.py build --debug --nightly
