#!/bin/sh

echo "Synchronizing pass"
pass git pull || exit 1
pass-truncate-history
pass git push
