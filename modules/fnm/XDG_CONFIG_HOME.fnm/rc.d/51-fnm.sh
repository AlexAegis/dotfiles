#!/bin/sh

# This file represents the default version that will automatically be used
# on first startup. On subsequent logins, it won't change it, only if it's
# unset for some reason
if ! command -v node &>/dev/null && command -v fnm; then
	fnm install latest
	fnm use latest
fi
