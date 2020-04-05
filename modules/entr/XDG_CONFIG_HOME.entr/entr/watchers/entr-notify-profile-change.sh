#!/bin/sh

echo "$HOME/.profile" | entr -p notify-send ".profile changed"
