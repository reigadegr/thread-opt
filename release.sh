cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")

export RUSTFLAGS="
    -Z mir-opt-level=2 \
    -Z share-generics=yes \
    -Z remap-cwd-prefix=. \
    -Z function-sections=yes \
    -Z dep-info-omit-d-target \
    -C default-linker-libraries \
    -C relocation-model=static \
    -C llvm-args=-vectorize-loops \
    -C llvm-args=-enable-misched \
    -C llvm-args=-enable-branch-hint \
    -C llvm-args=-enable-post-misched \
    -C llvm-args=-enable-dfa-jump-thread \
    -C link-args=-fomit-frame-pointer \
    -C link-arg=-Wl,--no-rosegment \
    -C link-args=-Wl,-O3,--gc-sections,--as-needed \
    -C link-args=-Wl,-z,norelro,-x,-z,noexecstack,--pack-dyn-relocs=android+relr,-s,--strip-all,--relax \
    -C llvm-args=-enable-ml-inliner=release \
    -C llvm-args=-ml-inliner-skip-policy=if-caller-not-cold \
    -C llvm-args=-ml-inliner-model-selector=arm64-mixed
" 

export RUSTFLAGS="$RUSTFLAGS -Z time-passes"

# export RUSTFLAGS="-C default-linker-libraries"
python3 ./make.py build --release --nightly
