#!/bin/sh

set -e

taplo fmt *.toml */*.toml */*/*.toml
export RUSTFLAGS="
    --cfg tokio_unstable
    -C link-arg=-fuse-ld=mold
    -C link-args=-Wl,--gc-sections,--as-needed
"

cargo fmt --all
# 运行 clippy
cargo clippy --workspace --all --all-targets --all-features --no-deps
cargo test --workspace
