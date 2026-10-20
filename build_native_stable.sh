#!/bin/sh

set -e

export RUSTFLAGS="
    --cfg tokio_unstable
    -C default-linker-libraries
    -C link-arg=-fuse-ld=mold
    -C symbol-mangling-version=v0
    -C link-arg=-Wl,--no-rosegment
    -C link-arg=-Wl,--sort-section=alignment
    -C link-args=-Wl,-O3,--gc-sections,--as-needed
    -C link-args=-Wl,-x,-z,noexecstack,--pack-dyn-relocs=relr,-s,--strip-all,--relax
"

if [ "$1" = "release" ] || [ "$1" = "r" ]; then
    cargo build -r
    bin=target/release/thread-opt
else
    cargo build
    bin=target/debug/thread-opt
fi

patchelf --remove-rpath "$bin"
if readelf -dW "$bin" | grep -q 'libtermux-platform-ns.so'; then
    patchelf --remove-needed libtermux-platform-ns.so "$bin"
fi
