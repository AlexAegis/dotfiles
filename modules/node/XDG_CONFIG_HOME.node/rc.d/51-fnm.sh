#!/bin/sh

# This file represents the default version that will automatically be used
# on first startup. On subsequent logins, it won't change it, only if it's
# unset for some reason
if ! command -v node 1>/dev/null 2>/dev/null && command -v fnm; then
	fnm install lts-latest
	fnm use lts-latest
fi
