#!/system/bin/sh
echo "Hello World!"
if [ ! -f /storage/emulated/0/Android/thread_opt.toml ]; then
    cp $MODPATH/thread_opt.toml /storage/emulated/0/Android/thread_opt.toml
fi

time=$(date "+%Y-%m-%d_%H:%M:%S")
cp -af /storage/emulated/0/Android/thread_opt.toml /storage/emulated/0/Android/thread_opt_"$time"backup.toml
cp -f $MODPATH/thread_opt.toml /storage/emulated/0/Android/thread_opt.toml

echo "仓库地址: https://github.com/reigadegr/thread-opt"
echo "适配游戏请截屏Scene帧率统计图 && 线程负载统计图"
echo "同时附上包名提交到仓库的issue处"
echo "祝使用愉快，觉得好用还请辛苦您给仓库点个Star，会支持作者写出更好的东西"
