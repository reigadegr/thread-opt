set -x
rm $(find ./src -name "*.bak")
rm ${0%/*}/*.bak ${0%/*}/.*.bak module/*.bak

rm $(find ./src -name "thread_opt")
rm ./thread_opt
uid=$(dumpsys package com.termux | grep appId | awk 'NR==1{print $1}' | cut -d '=' -f2)
chown -R $uid:$uid  ./src
chmod -R 0755 ./src
