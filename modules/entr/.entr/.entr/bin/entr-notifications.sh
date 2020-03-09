#!/bin/sh

for script in "$HOME"/.entr/notification/*.sh; do
	"$script" &
done
wait
