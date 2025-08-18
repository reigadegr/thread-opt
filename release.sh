cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")

export RUSTFLAGS="
    -Z validate-mir \
    -Z verify-llvm-ir \
    -Z mir-opt-level=1 \
    -Z share-generics=yes \
    -Z remap-cwd-prefix=. \
    -Z function-sections=yes \
    -Z dep-info-omit-d-target \
    -C default-linker-libraries \
    -C symbol-mangling-version=v0 \
    -C llvm-args=-enable-misched \
    -C llvm-args=-enable-post-misched \
    -C llvm-args=-enable-dfa-jump-thread \
    -C link-args=-fomit-frame-pointer \
    -C link-arg=-Wl,--no-rosegment \
    -C link-arg=-Wl,--sort-section=alignment \
    -C link-args=-Wl,-O1,--gc-sections,--as-needed \
    -C link-args=-Wl,-z,norelro,-x,-z,noexecstack,--pack-dyn-relocs=android+relr,-s,--strip-all,--relax
" 

export RUSTFLAGS="$RUSTFLAGS -Z time-passes"

# export RUSTFLAGS="-C default-linker-libraries"
python3 ./make.py build --release --nightly
