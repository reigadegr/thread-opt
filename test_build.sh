cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/debug -name "*thread-opt*")

clear_crash() {
    rm -rf $(find ./target/aarch64-linux-android/debug -name "*mimalloc*")
    rm -rf $(find ./target/aarch64-linux-android/debug -name "*ndk*")
}

python3 ./make.py build --debug --nightly || clear_crash


