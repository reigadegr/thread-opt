# 安装组件
# rustup component add llvm-tools-preview
pgo_path=$(realpath ./target/pgo-data)
# 生成插桩版本
# export LLVM_PROFILE_FILE="$pgo_path/default_*.profraw"

# RUSTFLAGS="-C default-linker-libraries -C profile-generate=$pgo_path" cargo +nightly build -Z build-std --release  --
# RUSTFLAGS="-C default-linker-libraries -C profile-generate=$pgo_path" cargo +nightly ndk -p 35 -t arm64-v8a build --target aarch64-linux-android -Z trim-paths -Z build-std --release  --
# 合并数据
# llvm-profdata merge -o $pgo_path/merged.profdata $pgo_path/*.profraw

# 应用PGO重新编译 Ok
if [ ! -z $(which dumpsys) ]; then
    pgo_path=$(realpath /data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/*"$pgo_path")
    rm -rf output
fi

RUSTFLAGS="-C default-linker-libraries \
 -C profile-use=$pgo_path/merged.profdata \
 -C link-args=-fomit-frame-pointer \
-C link-args=-Wl,--as-needed,--icf=all,-z,relro,--pack-dyn-relocs=android+relr,-x,-s,--strip-all,-z,now
" python3 ./make.py build --release --nightly -v
# cargo +nightly ndk -p 35 -t arm64-v8a build --target aarch64-linux-android -Z trim-paths -Z build-std --release  --
