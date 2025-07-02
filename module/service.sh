#!/system/bin/sh
MODDIR=${0%/*}
LOG=$MODDIR/log.txt
. "$MODDIR/utils.sh"

wait_until_login() {
    # in case of /data encryption is disabled
    while [ "$(getprop sys.boot_completed)" != "1" ]; do sleep 1; done
    # we doesn't have the permission to rw "/sdcard" before the user unlocks the screen
    until [ -d /sdcard/Android ]; do sleep 1; done
}

reset_freq(){
    for i in /sys/devices/system/cpu/cpu?/cpufreq/cpuinfo_max_freq; do
        lock_val $(cat $i) $(dirname "$i")/scaling_max_freq
    done

    for i in /sys/devices/system/cpu/cpu?/cpufreq/cpuinfo_min_freq; do
        lock_val $(cat $i) $(dirname "$i")/scaling_min_freq
    done
}

wait_until_login
if [ ! -L $MODDIR/thread_opt.toml ]; then
    profile_dir="/storage/emulated/0/Android/thread_opt"
    rm $MODDIR/thread_opt.toml
    ln -s "$profile_dir/thread_opt.toml" $MODDIR/thread_opt.toml
fi
stop oiface gameopt_hal_service-1-0 vendor.urcc-hal-aidl horae
killall -9 vendor.oplus.hardware.urcc-service vendor.oplus.hardware.gameopt-service oiface horae
mask_val_in_path "0" "/sys/module/cpufreq_bouncing/parameters/" "*"
reset_freq

killall -15 thread-opt; rm $LOG
chmod +x ${0%/*}/thread-opt
RUST_BACKTRACE=1 nohup $MODDIR/thread-opt >$LOG 2>&1 &
