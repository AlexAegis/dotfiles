#!/bin/sh

pass git push

target="${BACKUP_DIR:-"$HOME/backup/pass"}"
mkdir -p "$target"

pass-export-key "$target"
