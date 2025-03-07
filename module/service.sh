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

lock_val() {
    find "$2" -type f | while read -r file; do
        file="$(realpath "$file")"
        umount "$file"
        chown root:root "$file"
        chmod 0644 "$file"
        echo "$1" >"$file"
        chmod 0444 "$file"
    done
}

reset_freq(){
    for i in /sys/devices/system/cpu/cpu?/cpufreq/cpuinfo_max_freq; do
        lock_val $(cat $i) $(dirname "$i")/scaling_max_freq
    done

    for i in /sys/devices/system/cpu/cpu?/cpufreq/cpuinfo_min_freq; do
        lock_val $(cat $i) $(dirname "$i")/scaling_min_freq
    done
}


if [ "$(getprop sys.boot_completed)" != "1" ]; then
    contlict_remover "/data/adb/modules/AppOpt" "AppOpt"
    wait_until_login
    if [ ! -L $MODDIR/thread_opt.toml ]; then
        rm $MODDIR/thread_opt.toml
        ln -s /storage/emulated/0/Android/thread_opt.toml $MODDIR/thread_opt.toml
    fi
    stop oiface gameopt_hal_service-1-0 vendor.urcc-hal-aidl horae
    killall -9 vendor.oplus.hardware.urcc-service vendor.oplus.hardware.gameopt-service oiface horae
    reset_freq
fi

killall -15 thread-opt; rm $LOG
chmod +x ${0%/*}/thread-opt
RUST_BACKTRACE=1 nohup $MODDIR/thread-opt >$LOG 2>&1 &
