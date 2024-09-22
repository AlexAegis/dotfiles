#!/bin/sh

aura -Ayu --noconfirm # update AUR packages
aura -Bc --noconfirm # remove stale snapshots
aura check # diagnostics
