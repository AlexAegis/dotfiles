#!/bin/sh

#!/bin/sh

backup="${BACKUP_DIR:-"$HOME/backup"}/pass"

if [ -e "$backup" ]; then
	pass-import-key "${BACKUP_DIR:-"$HOME/backup/"}"
else
	echo "ERROR: Backup not found at: $backup"
fi
