#!/bin/sh



BACKUP_DIR="${BACKUP_DIR:-"$HOME/backup"}"
GNUPG_BACKUP_DIR="${BACKUP_DIR}/gnupg"
# XDG_CONFIG_HOME_GNUPG_BACKUP_DIR="${GNUPG_BACKUP_DIR}/xdg_config_home_gnupg"
EXPORT_BACKUP_DIR="${GNUPG_BACKUP_DIR}/gnupg_full_export"

LATEST_GNUPG_BACKUP_ARCHIVE=$(find "$HOME" -maxdepth 1 -type f -name 'gnupg-backup-*.7z' | sort | tail -n 1)

if [ -n "$LATEST_GNUPG_BACKUP_ARCHIVE" ]; then
	echo "Latest archive found: ${LATEST_GNUPG_BACKUP_ARCHIVE}"

	if [ -d "$GNUPG_BACKUP_DIR" ]; then
		printf "A backup already exists, can I delete '%s'? (Y/N): " "$GNUPG_BACKUP_DIR"
		read -r confirm

		case "$confirm" in
			[Yy])
				rm -rf "$GNUPG_BACKUP_DIR"
				echo "Old backup directory '$GNUPG_BACKUP_DIR' deleted."
				;;
			*)
				echo "Backup restoration cancelled, exiting.."
				exit 1
				;;
		esac
	fi

    echo "Extracting archive: $LATEST_GNUPG_BACKUP_ARCHIVE..."
	printf "Enter password: "
	stty -echo
	read password
	stty echo
	printf "\\n"

    7z x -p"${password}" "$LATEST_GNUPG_BACKUP_ARCHIVE" -o"$GNUPG_BACKUP_DIR"
else
    echo "No gnupg-backup archive found. Put the gnupg-backup-*.7z file into your home folder!"
    exit 0
fi

if [ -e "$EXPORT_BACKUP_DIR" ]; then

	gpg --import "$EXPORT_BACKUP_DIR/backup.pub.asc"
	gpg --import "$EXPORT_BACKUP_DIR/backup.priv.asc"
	gpg --import "$EXPORT_BACKUP_DIR/backup.sub_priv.asc"
	gpg --import-ownertrust "$EXPORT_BACKUP_DIR/backup.ownertrust.txt"

	# add gnupg keys to the pam-gnupg file
	./XDG_BIN_HOME.gnupg/setup-pam-gnupg
else
	echo "ERROR: Backup not found at: $EXPORT_BACKUP_DIR"
fi
