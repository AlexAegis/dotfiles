#!/bin/sh

backup="${BACKUP_DIR:-"$HOME/backup"}/.ssh"

if [ -e "$backup" ]; then
	cp -r "$backup" "$HOME"
else
	echo "ERROR: Backup not found at: $backup"
fi
