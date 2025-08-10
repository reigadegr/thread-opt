cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")

export RUSTFLAGS="
    -C default-linker-libraries \
    -Z external-clangrt \
    -Z macro-backtrace \
    -Z remap-cwd-prefix=. \
    -Z dep-info-omit-d-target \
    -Z merge-functions=aliases \
    -C llvm-args=-enable-ml-inliner=release \
    -C llvm-args=-regalloc-enable-advisor=release \
    -C llvm-args=-hot-cold-split=true \
    -C llvm-args=-enable-misched \
    -C llvm-args=-enable-scalable-autovec-in-streaming-mode \
    -C llvm-args=-inliner-interactive-include-default \
    -C llvm-args=-ml-inliner-model-selector=arm64-mixed \
    -C llvm-args=-ml-inliner-skip-policy=if-caller-not-cold \
    -C link-args=-fomit-frame-pointer \
    -C llvm-args=-mergefunc-use-aliases \
    -C llvm-args=-enable-shrink-wrap=1 \
    -C llvm-args=-enable-gvn-hoist \
    -C llvm-args=-enable-loop-versioning-licm \
    -C link-args=-Wl,-O3,--gc-sections,--as-needed \
    -C link-args=-Wl,--icf=all,-z,norelro,--pack-dyn-relocs=android+relr,-x,-s,--strip-all,-z,now
" 

# export RUSTFLAGS="-C default-linker-libraries"
python3 ./make.py build --release --nightly -v
