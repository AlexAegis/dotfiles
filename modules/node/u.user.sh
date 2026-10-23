#!/bin/sh

#shellcheck disable=SC1091
. "./0.user.sh"
. "./1.user.sh"

# update npm and other common package managers
# pnpm links its native binary in an install script
# yarn is listed only to stop npm's install-scripts warning
npm install -g --allow-scripts=pnpm,yarn npm yarn pnpm npm-check-updates turbo
