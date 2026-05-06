#!/bin/sh

tmpfile=$(mktemp)
rate-mirrors --allow-root --protocol https arch | grep -v '^#' > "$tmpfile"

if [ -s "$tmpfile" ]; then
	cp "$tmpfile" /etc/pacman.d/mirrorlist
	echo "Mirrorlist updated successfully ($(wc -l < "$tmpfile") mirrors)"
else
	echo "rate-mirrors produced no output, keeping existing mirrorlist" >&2
fi

rm -f "$tmpfile"
