#!/bin/sh

aura -A --noconfirm # update AUR packages
aura -Bc --noconfirm # remove stale snapshots
aura check # diagnostics
