cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/release -name "*thread-opt*")

export RUSTFLAGS="-C default-linker-libraries -C link-args=-Wl,--gc-sections -C link-args=-Wl,--as-needed -C link-args=-Wl,--icf=all -C link-args=-Wl -C link-args=-Wl,--pack-dyn-relocs=android+relr -C link-args=-Wl,-x -C link-args=-Wl,-s -C link-args=-Wl,--strip-all"

python3 ./make.py build --release --nightly
