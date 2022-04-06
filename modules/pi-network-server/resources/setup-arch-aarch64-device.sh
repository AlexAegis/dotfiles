#!/bin/sh

# Stop at any error
set -e

CACHE_DIR="${XDG_CACHE_HOME-$HOME/.cache}/rpi"
MOUNT_DIR="/mnt/rpi"
ARCH_64_URL="http://os.archlinuxarm.org/os/ArchLinuxARM-rpi-aarch64-latest.tar.gz"
ARCH_64_ROOTFS_FILE="${CACHE_DIR}/${ARCH_64_URL##*/}"

require_programs() {
	echo "Checking required programs..."
	IFS='
'

	not_found=''
	for program in "$@"; do
		if [ -z "$(command -v "${program}")" ]; then
			not_found="${not_found}${program} "
		fi
	done

	if [ -n "${not_found}" ]; then
		echo "Some required programs cannot be found:"
		echo "${not_found}"
		exit 1
	fi
}

require_programs lsblk mkfs.vfat mkfs.ext4 sfdisk wget bsdtar

device="$1"

if [ -z "$1" ]; then
	printf "Enter device to be formatted: "
	read -r device
fi

lsblk -o NAME,SIZE,RO,TYPE,MOUNTPOINTS,UUID,PARTUUID "${device}"

printf "Warning %s will be completely erased! Proceed? (y/n) " device
read -r confirm

if [ "${confirm}" != "y" ]; then
	echo "Look around with lsblk if unsure about your device"
	exit 1
fi

download_files() {
	echo "Making sure CACHE_DIR: $CACHE_DIR exists..."
	mkdir -p "$CACHE_DIR"
	(
		cd "$CACHE_DIR"
		for url in "$@"; do
			filename="${url##*/}"

			echo "Downloading $filename from $url"
			if [ -f "$CACHE_DIR/$filename" ]; then
				echo "$CACHE_DIR/$filename already exists, skipping."
			else
				echo "$CACHE_DIR/$filename doesn't exist. Start downloading:"
				wget "$url"
			fi
		done
	)
}

download_files "${ARCH_64_URL}"

## FORMATTING

echo "Now the tool will create a 256M partition for /boot and an ext4 " \
	"partition for /root"
printf "Do you want a third partition? (y/n) "
read -r use_third_partition


if [ -n "$use_third_partition" ]; then
	printf "Enter the size of the second (/root) partition, the rest will be formatted as the third partition (16G): "
	read -r second_partition_size
	second_partition_size="${second_partition_size:=16G}"

	printf "Enter the mount point of the third partition (defaults to /home): "
	read -r third_partition_mount_point
	third_partition_mount_point="${third_partition_mount_point:=/home}"

	if [ "${third_partition_mount_point##/}" = "$third_partition_mount_point" ]; then
		echo "The third partitions mounting point must be an absolute path!"
		exit 1
	fi
fi


sfdisk_script="
label: dos
label-id: 0x4b70f675
device: ${device}
unit: sectors
sector-size: 512

${device}1 : start=                  2048, size=                524288, type=c
${device}2 : start=                526336, size= ${second_partition_size}, type=83
"

if [ -n "${second_partition_size}" ]; then
sfdisk_script="${sfdisk_script}
${device}3 : start=                      , size=                      , type=83
"
fi

echo "Proceed using this sfdisk script:
${sfdisk_script}"

sfdisk "${1}" << EOF
${sfdisk_script}
EOF

echo "Partitioning first partition as FAT32..."
mkfs.vfat -F32 "${device}1"
echo "Partitioning second (/root) partition as EXT4..."
mkfs.ext4 "${device}2"

if [ -n "$second_partition_size" ]; then
	echo "Partitioning third ($third_partition_mount_point) partition as EXT4..."
	mkfs.ext4 "${device}3"
fi

install() {
	echo "Mounting ${1}2 to ${MOUNT_DIR}"
	mount "${1}2" "${MOUNT_DIR}"
	mkdir -p "${MOUNT_DIR}/boot"
	echo "Mounting ${1}1 to ${MOUNT_DIR}/boot"
	mount "${1}1" "${MOUNT_DIR}/boot"

	if [ -n "${use_third_partition}" ] && [ -n "${third_partition_mount_point}" ]; then
		echo "Mounting ${1}3 to ${MOUNT_DIR}${third_partition_mount_point}"
		mkdir -p "${MOUNT_DIR}${third_partition_mount_point}"
		mount "${1}3" "${MOUNT_DIR}${third_partition_mount_point}"
	fi

	echo "Extracting install file ${ARCH_64_ROOTFS_FILE} into ${MOUNT_DIR}"
	bsdtar -xpf "${ARCH_64_ROOTFS_FILE}" -C "${MOUNT_DIR}"
}

apply_fixes() {
	echo "!!! The root partition has to be hardcoded as of 2022-04-06 " \
		 "On the rpi4 the sd-cards second partition would be found at " \
		 "/dev/mmcblk0p2 while a USB devices would be at /dev/sda2"
	printf "Is this device an sd-card? (y/n) "
	read -r is_sd_card

	if [ "${is_sd_card}" = 'y' ]; then
		device_at_pi='/dev/mmcblk0p'
	else
		device_at_pi='/dev/sda'
	fi

	# Thank you so much kyoshiro for figuring this out!
	# <https://archlinuxarm.org/forum/viewtopic.php?f=67&t=15422&start=20#p68473>
	# Compared to the original boot.txt $`{fdt_addr_r}` is replaced with
	# `${fdt_addr}` in the lines starting with `booti`
	# and `root` is hardcoded to use the device name instead of UUID=/PARTUUID=
	# As that doesn't seem to
	boottxt="# After modifying, run ./mkscr

# Set root partition to the second partition of boot device
part uuid \${devtype} \${devnum}:2 uuid

setenv bootargs earlyprintk console=ttyS1,115200 console=tty0 root=${device_at_pi}2 rw rootwait smsc95xx.macaddr=\"\${usbethaddr}\"

if load \${devtype} \${devnum}:\${bootpart} \${kernel_addr_r} /Image; then
  if load \${devtype} \${devnum}:\${bootpart} \${fdt_addr_r} /dtbs/\${fdtfile}; then
    if load \${devtype} \${devnum}:\${bootpart} \${ramdisk_addr_r} /initramfs-linux.img; then
      booti \${kernel_addr_r} \${ramdisk_addr_r}:\${filesize} \${fdt_addr};
    else
      booti \${kernel_addr_r} - \${fdt_addr};
    fi;
  fi;
fi
"

	fstab="${device_at_pi}1		/boot		vfat		defaults		0	0
${device_at_pi}2		/		ext4		defaults		0	1
"

	if [ -n "${third_partition_mount_point}" ]; then
		fstab="${fstab}
${device_at_pi}3		${third_partition_mount_point}		ext4		defaults		0	2
"
	fi

	if [ ! -f "${MOUNT_DIR}/boot/boot.txt.bak" ]; then
		echo "No boot.txt backup found, backing up original as boot.txt.bak..."
		cp "${MOUNT_DIR}/boot/boot.txt" "${MOUNT_DIR}/boot/boot.txt.bak"
	fi

	echo "Writing patched boot.txt to ${MOUNT_DIR}/boot/boot.txt"
	echo "${boottxt}" > "${MOUNT_DIR}/boot/boot.txt"
	echo "Regenerating boot.scr..."
	(
		cd "${MOUNT_DIR}/boot"
		./mkscr
	)
	echo "Writing fstab to ${MOUNT_DIR}/etc/fstab"
	echo "${fstab}" > "${MOUNT_DIR}/etc/fstab"
}

install "${device}"
apply_fixes

echo "Syncing and unmounting ${MOUNT_DIR}..."
sync
umount -R "${MOUNT_DIR}"

echo "Some tips:

!!! USB keyboards likely wont work so try using SSH! (ssh alarm@192.168.0.XXX)
!!! It will work after the first system upgrade
(You can check the ip of it in your routers admin page! Consider giving it a
fix IP in your DHCP config!)

Default user:passwords = alarm:alarm, root:root

You wont be able to ssh directly as root, ssh as alarm and then use 'su -' to
switch to root

!!! Start with initializing the keyring:
pacman-key --init
pacman-key --populate archlinuxarm

And a system upgrade (And might aswell install some basic stuff):
pacman -Syu sudo vim git base-devel

Then do a restart to see if it still works!
"

echo "

SUCCESS! Your pi crust is now done! Now go ahead and make some filling!"
