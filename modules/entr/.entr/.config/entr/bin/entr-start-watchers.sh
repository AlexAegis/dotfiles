#!/bin/sh

for script in "$HOME"/.config/entr/watchers/*.sh; do
	"$script" &
done
