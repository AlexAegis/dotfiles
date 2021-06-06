#!/bin/sh

export BACKUP_DIR="${BACKUP_DIR:-"$HOME/backup"}/gnupg"

if [ -e "$BACKUP_DIR" ]; then
	cp -r "$BACKUP_DIR" "$XDG_CONFIG_HOME"

	gpg --import "$BACKUP_DIR/backup.pub.asc"
	gpg --import "$BACKUP_DIR/backup.priv.asc"
	gpg --import "$BACKUP_DIR/backup.sub_priv.asc"
	gpg --import-ownertrust "$BACKUP_DIR/backup.ownertrust.txt"

	# add gnupg keygrips to the pam-gnupg file
	./XDG_BIN_HOME.gnupg/setup-pam-gnupg
else
	echo "ERROR: Backup not found at: $BACKUP_DIR"
fi
