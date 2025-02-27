cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/debug -name "*thread-opt*")

python3 ./make.py build --debug --nightly || rm -rf $(find ./target/aarch64-linux-android/debug -name "*mimalloc*")
