#!/bin/sh

if timelock reflector_mirrorlist; then
	tmpfile=$(mktemp)
	reflector --latest 20 --protocol https --sort rate --save "$tmpfile"

	if [ -s "$tmpfile" ]; then
		cp "$tmpfile" /etc/pacman.d/mirrorlist
		echo "Mirrorlist updated successfully ($(grep -c '^Server' "$tmpfile") mirrors)"
		# Only locked on success so a failed run retries on the next update
		timelock reflector_mirrorlist 'month'
	else
		echo "reflector produced no output, keeping existing mirrorlist" >&2
	fi

	rm -f "$tmpfile"
else
	echo "The reflector_mirrorlist timelock is locked,\
 it unlocks at $(timelock -p reflector_mirrorlist)"
fi
