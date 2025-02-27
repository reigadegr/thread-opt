#!/system/bin/sh
MODDIR=${0%/*}
LOG=$MODDIR/log.txt

wait_until_login() {
    # in case of /data encryption is disabled
    while [ "$(getprop sys.boot_completed)" != "1" ]; do sleep 1; done
    # we doesn't have the permission to rw "/sdcard" before the user unlocks the screen
    until [ -d /sdcard/Android ]; do sleep 1; done
}

contlict_remover() {
    [ ! -d "$1" ] && return
    chattr -R -ia $1
    rm -rf $1
    killall -9 $2
}

if [ "$(getprop sys.boot_completed)" != "1" ]; then
    contlict_remover "/data/adb/modules/AppOpt" "AppOpt"
    wait_until_login
    stop oiface gameopt_hal_service-1-0 vendor.urcc-hal-aidl horae
    killall -9 vendor.oplus.hardware.urcc-service vendor.oplus.hardware.gameopt-service oiface horae
fi

killall -15 thread-opt; rm $LOG
chmod +x ${0%/*}/thread-opt
RUST_BACKTRACE=1 nohup $MODDIR/thread-opt >$LOG 2>&1 &
