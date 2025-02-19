sed -i 's/edition = "2021"/edition = "2024"/g' Cargo.toml
cargo fmt
cargo clippy || exit 0
sed -i 's/edition = "2024"/edition = "2021"/g' Cargo.toml
