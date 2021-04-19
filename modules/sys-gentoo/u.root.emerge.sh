#!/bin/sh

emerge --sync
emaint sync -R # Sync all repositories
emerge --update --changed-use --deep @world
