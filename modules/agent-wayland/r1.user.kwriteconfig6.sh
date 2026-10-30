#!/bin/sh

rule=87047328-2cc4-4879-98c9-2b921a1e5ab0
file=${XDG_CONFIG_HOME:-$HOME/.config}/kwinrulesrc

# Every key of the group, also keys added in System Settings: a group without
# wmclass would match every window
sed -n "/^\[$rule\]\$/,/^\[/{/^\[/d;s/=.*//p}" "$file" | while IFS= read -r key; do
	kwriteconfig6 --file kwinrulesrc --group "$rule" --key "$key" --delete || exit 1
done || exit 1

for list in Order rules; do
	value=$(kreadconfig6 --file kwinrulesrc --group General --key "$list")
	[ -n "$value" ] || continue
	kwriteconfig6 --file kwinrulesrc --group General --key "$list" \
		"$(printf '%s\n' "$value" | tr ',' '\n' | grep -vx "$rule" | paste -sd ,)" || exit 1
done

dbus-send --session --type=method_call --dest=org.kde.KWin /KWin \
	org.kde.KWin.reconfigure 2>/dev/null || true
