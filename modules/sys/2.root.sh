#!/bin/sh

setting='set bell-style none'
file='/etc/inputrc'

sed -i "s/^# *\($setting\)/\1/g" "$file" # Uncomment

if ! grep -q -x "$setting" "$file"; then # If still not present, append
	echo "$setting" >> "$file"
fi
