#!/bin/sh

echo "$HOME/.profile" | entr -p notify-send "Env changed"
