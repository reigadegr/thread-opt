cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")

export RUSTFLAGS="
    -Z mir-opt-level=2
    -Z dylib-lto=yes
    -Z inline-mir=yes
    -Z fewer-names=yes
    -Z share-generics=yes
    -Z remap-cwd-prefix=.
    -Z function-sections=yes
    -Z dep-info-omit-d-target
    -Z flatten-format-args=yes
    -Z saturating-float-casts=yes
    -Z mir-enable-passes=+Inline
    -Z precise-enum-drop-elaboration=yes
    -C default-linker-libraries
    -C relro-level=none
    -C code-model=small
    -C relocation-model=pie
    -C symbol-mangling-version=v0
    -C llvm-args=-fp-contract=off
    -C llvm-args=-enable-misched
    -C llvm-args=-enable-post-misched
    -C llvm-args=-enable-dfa-jump-thread
    -C link-arg=-Wl,--no-rosegment
    -C link-arg=-Wl,--sort-section=alignment
    -C link-args=-Wl,-O2,--gc-sections,--as-needed
    -C link-args=-Wl,-x,-z,noexecstack,--pack-dyn-relocs=android+relr,-s,--strip-all,--relax
" 

export RUSTFLAGS="$RUSTFLAGS -Z time-passes"

python3 ./make.py build --release --nightly
# cargo +nightly ndk --platform 35 -t arm64-v8a build --target aarch64-linux-android -Z trim-paths -Z build-std --release
