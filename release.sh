cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")

export RUSTFLAGS="
-C default-linker-libraries \
-C link-args=-fomit-frame-pointer \
-C link-args=-Wl,--as-needed,--icf=all,-z,relro,--pack-dyn-relocs=android+relr,-x,-s,--strip-all,-z,now
"

# export RUSTFLAGS="-C default-linker-libraries"
python3 ./make.py build --release --nightly
