#!/bin/sh

export RUSTFLAGS="
    --cfg tokio_unstable
    -C default-linker-libraries
    -C relro-level=full
    -C link-arg=-fuse-ld=mold
    -C symbol-mangling-version=v0
    -C llvm-args=-fp-contract=off
    -C llvm-args=-enable-misched
    -C llvm-args=-enable-post-misched
    -C llvm-args=-enable-dfa-jump-thread
    -C link-arg=-Wl,--no-rosegment
    -C link-arg=-Wl,--sort-section=alignment
    -C link-args=-Wl,-O3,--gc-sections,--as-needed
    -C link-args=-Wl,-x,-z,noexecstack,--pack-dyn-relocs=relr,-s,--strip-all,--relax
"

if [ "$1" = "release" ] || [ "$1" = "r" ]; then
    cargo build -r
else
    cargo build
fi
