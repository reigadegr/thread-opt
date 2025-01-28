#!/system/bin/sh
MODDIR=${0%/*}
LOG=$MODDIR/log.txt

wait_until_login() {
    # in case of /data encryption is disabled
    while [ "$(getprop sys.boot_completed)" != "1" ]; do sleep 1; done
    # we doesn't have the permission to rw "/sdcard" before the user unlocks the screen
    until [ -d /sdcard/Android ]; do sleep 1; done
}

wait_until_login
killall -15 thread-opt; rm $LOG
chmod +x ${0%/*}/thread-opt
RUST_BACKTRACE=1 nohup $MODDIR/thread-opt >$LOG 2>&1 &
