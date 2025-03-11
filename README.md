# thread-opt
> thread-opt是使用Rust语言构建的自定义线程规则的模块，通过硬亲和的方式把线程绑定到指定的CPU核心，以优化游戏效果，通过libc库的sched_setaffinity函数实现

[stars-badge]: https://img.shields.io/github/stars/reigadegr/thread-opt?style=for-the-badge&logo=github
[stars-url]: https://github.com/reigadegr/thread-opt
[ci-badge]: https://img.shields.io/github/actions/workflow/status/reigadegr/thread-opt/ci.yml?style=for-the-badge&label=CI%20Build&logo=githubactions
[ci-url]: https://github.com/reigadegr/thread-opt/actions/workflows/ci.yml
[release-badge]: https://img.shields.io/github/v/release/reigadegr/thread-opt?style=for-the-badge&logo=rust
[release-url]: https://github.com/reigadegr/thread-opt/releases/latest
[download-badge]: https://img.shields.io/github/downloads/reigadegr/thread-opt/total?style=for-the-badge
[download-url]: https://github.com/reigadegr/thread-opt/releases/latest

从2.0.0版本开始，对三种放置规则全面引入了配置文件的支持

默认配置在大部分场景下已足够优秀，无特殊需求无需修改

当然，你也可以对现有规则进行修改，且任何人可以发布二改的配置文件

配置文件位置: 
```txt
/data/adb/modules/thread_opt/thread_opt.toml
```

```txt
/storage/emulated/0/Android/thread_opt.toml
```

- 程序开始运行将读取本机cgroup。使得一份配置文件，能够在全平台通用

- 每个规则你都可以定义很多组，但是配置文件大小不得超过65536字节(64kb)

- 修改完配置文件需要重新执行service.sh以应用更改。执行完记得看一眼log.txt，别改错了

### `规则1(comm_match)`: 
- 单纯进行线程前缀的匹配，依次遍历Top,Only6,Only7,Middle,Background数组。若对应数组内某个元素是你当前正在匹配的线程前缀，那么将当前线程按照其对应的规则绑定。如果都不符合，按照Middle方案处理

### `规则2(usage_top1)`: 
- 根据配置文件指定的线程前缀名，找出具有此前缀全部的线程，通过读负载的形式找出一个负载最高的线程，绑定到cpu6或者cpu7。随后，对于其他全部的线程，按照`规则1(comm_match)`的方案处理其余线程

### `规则3(usage_top2)`: 有两种小规则:
- 只写了`max_comm`字段: 将找出具有此前缀全部的线程，通过读负载的形式找出前两个负载最高的线程，依次应用Only7和Only6。其余线程绑定到`Middle` + `Background`组成的大集合内

- 也写了`second_comm`字段: 将分别找出具有max_comm，second_comm前缀的线程，分别计算一个负载最高的线程。把max_comm计算出的tid应用为Only7，把second_comm计算出的tid应用为Only6。其余线程绑定到`Middle` + `Background`组成的大集合内

> 若能帮到你，还请为项目点一颗star⭐(作者的小心思(x))

## 问答
- Q: 什么是`Top,Middle,Background,Only6,Only7`?

- A: 这些是几个预定义的枚举，其中Top，Middle，Background为本机的大，中，小核集群。如果本机只有两个集群，则令Middle = Background。具体可以去`/data/adb/modules/thread_opt/log.txt`查看。Only7为把线程单独放置到cpu7，Only6为把线程单独放置到cpu6

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
