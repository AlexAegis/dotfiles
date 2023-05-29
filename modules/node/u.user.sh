#!/bin/sh

#shellcheck disable=SC1091
. "./0.user.sh"
. "./1.user.sh"

# update npm and other common package managers
npm install -g npm yarn pnpm npm-check-updates turbo
