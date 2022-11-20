#!/bin/sh

#shellcheck disable=SC1091
. "./0.user.sh"
. "./1.user.sh"

# update npm and other common package managers
npm install -g npm
npm install -g yarn
npm install -g pnpm
