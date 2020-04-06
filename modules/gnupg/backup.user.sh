#!/bin/sh

cp -r "$XDG_CONFIG_HOME/gnupg" "${BACKUP_DIR:-"$HOME/backup"}"
