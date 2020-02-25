#!/bin/sh

# TODO: make function

LINE='export "$PATH:~/tools/"'
FILE='~/filename.conf'
grep -qF -- "$LINE" "$FILE" || echo "$LINE" >> "$FILE"
