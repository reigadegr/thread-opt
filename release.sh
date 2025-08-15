cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")

export RUSTFLAGS="
    -C default-linker-libraries \
    -Z plt=no \
    -Z mir-opt-level=4 \
    -Z share-generics=yes \
    -Z remap-cwd-prefix=. \
    -Z function-sections=yes \
    -Z dep-info-omit-d-target \
    -C llvm-args=-enable-misched \
    -C llvm-args=-hot-cold-split=true \
    -C llvm-args=-aggressive-ext-opt \
    -C llvm-args=-enable-post-misched \
    -C llvm-args=-enable-shrink-wrap=1 \
    -C llvm-args=-mergefunc-use-aliases \
    -C llvm-args=-enable-dfa-jump-thread \
    -C llvm-args=-enable-loopinterchange \
    -C llvm-args=-extra-vectorizer-passes \
    -C llvm-args=-enable-ml-inliner=release \
    -C llvm-args=-enable-loop-versioning-licm \
    -C llvm-args=-regalloc-enable-advisor=release \
    -C llvm-args=-enable-ext-tsp-block-placement \
    -C llvm-args=-ml-inliner-skip-policy=if-caller-not-cold \
    -C llvm-args=-ml-inliner-model-selector=arm64-mixed \
    -C llvm-args=-enable-scalable-autovec-in-streaming-mode \
    -C link-arg=-Wl,--no-rosegment \
    -C link-args=-fomit-frame-pointer \
    -C link-args=-Wl,-O3,--gc-sections,--as-needed \
    -C link-args=-Wl,--icf=all,-z,norelro,--pack-dyn-relocs=android+relr,-x,-s,--strip-all,--relax
" 

export RUSTFLAGS="$RUSTFLAGS -Z time-passes"

# export RUSTFLAGS="-C default-linker-libraries"
python3 ./make.py build --release --nightly
