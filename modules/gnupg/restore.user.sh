#!/bin/sh

backup="${BACKUP_DIR:-"$HOME/backup"}/gnupg"

if [ -e "$backup" ]; then
	cp -r "$backup" "$XDG_CONFIG_HOME"
else
	echo "ERROR: Backup not found at: $backup"
fi
