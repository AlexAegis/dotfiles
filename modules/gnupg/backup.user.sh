#!/bin/sh

export BACKUP_DIR="${BACKUP_DIR:-"$HOME/backup"}/gnupg"

cp -r "$XDG_CONFIG_HOME/gnupg" "$BACKUP_DIR"

gpg --export --armor > "$BACKUP_DIR/backup.pub.asc"
gpg --export-secret-keys --armor > "$BACKUP_DIR/backup.priv.asc"
gpg --export-secret-subkeys --armor > "$BACKUP_DIR/backup.sub_priv.asc"
gpg --export-ownertrust > "$BACKUP_DIR/backup.ownertrust.txt"
