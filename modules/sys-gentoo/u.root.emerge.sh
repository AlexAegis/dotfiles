#!/bin/sh

emerge --sync
emaint sync -A # Sync all repositories
emerge --update --changed-use --deep @world
