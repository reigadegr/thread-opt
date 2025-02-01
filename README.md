# thread-opt
通过硬亲和的方式把线程绑定到指定的CPU核心，以优化游戏效果。通过libc库的sched_setaffinity函数实现

### 编译方法
#### 环境搭建(任意完整的Linux环境即可，使用Termux的Arch Linux proot做示范)
- 下载容器
```shell
pkg install proot -y; pkg install proot-distro -y; proot-distro add archlinux
```

- 登录容器

```shell
proot-distro login archlinux
```

- 更新源

```shell
yes | pacman -Sy
```

- 安装依赖
```shell
yes | pacman -S llvm clang python glibc make cmake
```

- 安装rust
> 默认为nightly，default

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --profile default -y
```
> 此时需重启终端，来设置环境变量。其他方法也可以

```shell
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

rustup component add rust-src

cargo install cargo-ndk
```

- 下载android NDK
- aarch64架构:(termux环境请使用此ndk)
  https://github.com/Lzhiyong/termux-ndk/releases

- x86_64架构：
  https://github.com/android/ndk/releases/latest

- 下载完毕后，把下载好的zip改名为ndk.zip，随后按照以下代码设置
```shell
mkdir ~/ndk_temp 2>/dev/null
unzip ndk.zip -d ~/ndk_temp
mv ~/ndk_temp/*/* ~/ndk_temp
```
- 随后设置环境变量
```shell
export ANDROID_NDK_HOME=$(realpath ~/ndk_temp)
export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME
```
全部设置完毕后，执行`sh release.sh` 即可

### 致谢
- [fas-rs] https://github.com/shadow3aaa/fas-rs

> 采用了其大量方案

### 说明
本项目采用GPL v3协议，如果使用了本项目相关内容请您开源。
如果能帮到你，还请为本项目添加Start。
如果需要对游戏适配，欢迎提issue，并附上包名和Scene线程负载统计图。
