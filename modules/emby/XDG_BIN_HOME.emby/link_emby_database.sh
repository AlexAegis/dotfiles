#!/bin/sh

# Moves the data folder (/var/lib/emby or $LINK_EMBY_DATABASE_DATA_DIR) to the
# absolute folder $1 through $1.partial, then links it to $1
# - $1 exists: the data folder moves into it as prev-server-backup-<date>
# - data folder is a link: only the link changes, the data stays

case $1 in
	/*) ;;
	*)
		echo 'Provide the target folder as an absolute path' >&2
		exit 1
		;;
esac

emby_dir=$(realpath -ms -- "${LINK_EMBY_DATABASE_DATA_DIR:-/var/lib/emby}") ||
	exit 1
target_dir=$(realpath -ms -- "$1") || exit 1
staging_dir="${target_dir}.partial"

case $target_dir in
	/ | "$emby_dir" | "$emby_dir"/*)
		printf 'Target %s cannot hold the emby data\n' "$1" >&2
		exit 1
		;;
esac

if [ -e "${target_dir}" ] && [ ! -d "${target_dir}" ]; then
	printf 'Target %s is not a folder\n' "${target_dir}" >&2
	exit 1
fi

if [ -e "${staging_dir}" ] || [ -L "${staging_dir}" ]; then
	printf '%s is left from a failed move, check and remove it\n' \
		"${staging_dir}" >&2
	exit 1
fi

if [ -L "${emby_dir}" ]; then
	old_link=$(readlink -- "${emby_dir}") || exit 1
	rm "${emby_dir}" || exit 1
	if ! mkdir -p "${target_dir}" || [ ! -d "${target_dir}" ] ||
		[ -e "${emby_dir}" ] || [ -L "${emby_dir}" ]; then
		printf 'Target %s cannot be created, or leads through the data link\n' \
			"$1" >&2
		if [ -d "${emby_dir}" ] && [ ! -L "${emby_dir}" ]; then
			find "${emby_dir}" -depth -type d -exec rmdir -- {} +
		fi
		ln -sT "${old_link}" "${emby_dir}" ||
			printf 'Could not restore the link %s -> %s\n' "${emby_dir}" \
				"${old_link}" >&2
		exit 1
	fi
elif [ -d "${emby_dir}" ]; then
	if [ -d "${target_dir}" ]; then
		bak_date="$(date +'%Y-%m-%d-%H:%M:%S')"
		mv "${emby_dir}" "${target_dir}/prev-server-backup-${bak_date}" ||
			exit 1
	else
		mkdir -p "$(dirname "${target_dir}")" || exit 1
		mv "${emby_dir}" "${staging_dir}" || exit 1
		mv "${staging_dir}" "${target_dir}" || exit 1
	fi
elif [ -e "${emby_dir}" ]; then
	printf 'Data folder %s is not a folder or a link\n' "${emby_dir}" >&2
	exit 1
fi

mkdir -p "${target_dir}" || exit 1
ln -sT "${target_dir}" "${emby_dir}" || exit 1

chown -R emby:emby "${emby_dir}"
chown -R emby:emby "${target_dir}"
