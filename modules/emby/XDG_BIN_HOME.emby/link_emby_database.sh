#!/bin/sh

# This script needs a target folder where you want your emby data to be placed
# The source folder is always /var/lib/emby
# If /var/lib/emby does not exist, the script will just create a symlink from
# it to the target folder
# If /var/lib/emby does exist, but the target folder does not, it will first
# move it there and then symlink it.
# If both exist, a backup of /var/lib/emby will be created next to the target
# and then the script will proceed as there were no data before.

emby_dir='/var/lib/emby'
target_dir="$1"

if [ -z "${target_dir}" ]; then
	echo 'Provide a target directory' || exit 1
fi

# only back it up if its not already a symlink
if [ -d "${emby_dir}" ] && \
   [ -d "${target_dir}" ] && \
   [ ! -L "${emby_dir}" ]
then
	bak_date="$(date +'%Y-%M-%d-%H:%M:%S')"
	mv "${emby_dir}" "${target_dir}/prev-server-backup-${bak_date}"
fi

rm -rf "${emby_dir}"
mkdir -p "${target_dir}"

(
	cd "/var/lib/" || exit 1
	ln -s "${target_dir}" emby
)

chown -R emby:emby "${emby_dir}"
chown -R emby:emby "${target_dir}"
