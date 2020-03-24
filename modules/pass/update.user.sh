#!/bin/sh

echo "Synchronizing pass"
pass git pull
pass-truncate-history
pass git push
