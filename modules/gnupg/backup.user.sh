#!/bin/sh

BACKUP_DIR="${BACKUP_DIR:-"$HOME/backup"}"
GNUPG_BACKUP_DIR="${BACKUP_DIR}/gnupg"
XDG_CONFIG_HOME_GNUPG_BACKUP_DIR="${GNUPG_BACKUP_DIR}/xdg_config_home_gnupg"
EXPORT_BACKUP_DIR="${GNUPG_BACKUP_DIR}/gnupg_full_export"

if [ -d "$GNUPG_BACKUP_DIR" ]; then
    printf "A backup already exists, can I delete '%s'? (Y/N): " "$GNUPG_BACKUP_DIR"
    read -r confirm

    case "$confirm" in
        [Yy])
            rm -rf "$GNUPG_BACKUP_DIR"
            echo "Old backup directory '$GNUPG_BACKUP_DIR' deleted."
            ;;
        *)
            echo "Backup creation cancelled, exiting.."
			exit 1
            ;;
    esac
fi

mkdir -p "$XDG_CONFIG_HOME_GNUPG_BACKUP_DIR"
mkdir -p "$EXPORT_BACKUP_DIR"

cp -r "$XDG_CONFIG_HOME/gnupg" "$XDG_CONFIG_HOME_GNUPG_BACKUP_DIR"
# remove symbolic links
find "$XDG_CONFIG_HOME_GNUPG_BACKUP_DIR" -type l -exec rm -f {} \;

echo "Exporting public keys..."
gpg --export --armor > "$EXPORT_BACKUP_DIR/backup.pub.asc"
echo "Exporting secret keys..."
gpg --export-secret-keys --armor > "$EXPORT_BACKUP_DIR/backup.priv.asc"
echo "Exporting secret subkeys..."
gpg --export-secret-subkeys --armor > "$EXPORT_BACKUP_DIR/backup.sub_priv.asc"
echo "Exporting ownertrust..."
gpg --export-ownertrust > "$EXPORT_BACKUP_DIR/backup.ownertrust.txt"

TODAY=$(date --iso-8601=date)

if command -v 7z > /dev/null; then
	printf "Creating password protected 7z archive, enter password: "
	stty -echo
	read password
	stty echo
	printf "\\n"
	7z a -p"${password}" -mhe=on "${HOME}"/gnupg-backup-"${TODAY}".7z "$GNUPG_BACKUP_DIR"/*
	echo "Removing $GNUPG_BACKUP_DIR..."
	rm -rf "$GNUPG_BACKUP_DIR"
else
	echo "You may want to create a password protected archive out of this (keep the space before 7z, so your password won't be saved into your history)"
	echo " 7z a -p'<password>' -mhe=on ${HOME}/gnupg-backup-${TODAY}.7z $GNUPG_BACKUP_DIR/*"
fi

