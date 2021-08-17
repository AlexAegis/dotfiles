#!/bin/sh

# add missing gpg key
curl -sS https://download.spotify.com/debian/pubkey_0D811D58.gpg | gpg --import -
paru -Syu --needed --noconfirm spotify
