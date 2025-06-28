#!/system/bin/sh

mkdir -p /dev/mount_masks

# $1:value $2:path
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

# $1:value $2:path $3:filename
# $1:value $2:path $3:subdir $4:filename
lock_val_in_path() {
    if [ "$#" = "4" ]; then
        find "$2/" -path "*$3*" -name "$4" -type f | while read -r file; do
            lock_val "$1" "$file"
        done
    else
        find "$2/" -name "$3" -type f | while read -r file; do
            lock_val "$1" "$file"
        done
    fi
}

# $1:value $2:path
mask_val() {
    find "$2" -type f | while read -r file; do
        file="$(realpath "$file")"
        lock_val "$1" "$file"

        TIME="$(date "+%s%N")"
        echo "$1" >"/dev/mount_masks/mount_mask_$TIME"
        mount --bind "/dev/mount_masks/mount_mask_$TIME" "$file"
        restorecon -R -F "$file" >/dev/null 2>&1
    done
}

# $1:value $2:path $3:filename
# $1:value $2:path $3:subdir $4:filename
mask_val_in_path() {
    if [ "$#" = "4" ]; then
        find "$2/" -path "*$3*" -name "$4" -type f | while read -r file; do
            mask_val "$1" "$file"
        done
    else
        find "$2/" -name "$3" -type f | while read -r file; do
            mask_val "$1" "$file"
        done
    fi
}

# $1:min
lock_perfhal_min() {
    mask_val "0:$1 1:$1 2:$1 3:$1 4:$1 5:$1 6:$1 7:$1" "/sys/kernel/msm_performance/parameters/cpu_min_freq"
}

# $1:max
lock_perfhal_max() {
    mask_val "0:$1 1:$1 2:$1 3:$1 4:$1 5:$1 6:$1 7:$1" "/sys/kernel/msm_performance/parameters/cpu_max_freq"
}

# $1:min $2:max
lock_perfhal() {
    lock_perfhal_min "$1"
    lock_perfhal_max "$2"
}

wait_until_boot_complete() {
    # in case of /data encryption is disabled
    resetprop -w sys.boot_completed 0
}

wait_until_login() {
    # in case of the user unlocked the screen
    until [ -d "/data/data/android" ]; do
        sleep 1
    done
}

set_governor() {
    lock_val_in_path "$1" "/sys/devices/system/cpu/cpufreq" "scaling_governor"
}

disable_corectl() {
    lock_val_in_path "1" "/sys/devices/system/cpu" "core_ctl" "enable"
    lock_val_in_path "99" "/sys/devices/system/cpu" "core_ctl" "min_cpus"
    lock_val_in_path "99" "/sys/devices/system/cpu" "core_ctl" "max_cpus"
    lock_val_in_path "0" "/sys/devices/system/cpu" "core_ctl" "enable"
}

exec_system() {
    eval "$1" </dev/null 2>&1 | cat
}
