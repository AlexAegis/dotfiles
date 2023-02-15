#!/bin/sh

# add missing gpg key
curl -sS https://download.spotify.com/debian/pubkey_7A3A762FAFD4A51F.gpg | gpg --import -
paru -Syu --needed --noconfirm spotify
