#!/bin/sh
export RUSTFLAGS="
    --cfg tokio_unstable
    -C link-arg=-fuse-ld=mold
    -C link-args=-Wl,--gc-sections,--as-needed
"
cargo clippy --workspace --fix --allow-dirty --allow-staged --all --all-targets --all-features --no-deps
