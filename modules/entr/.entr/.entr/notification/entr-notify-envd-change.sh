#!/bin/sh

while true; do
	find "$HOME"/.env.d/* | entr -d -p notify-send "env.d changed"
done
