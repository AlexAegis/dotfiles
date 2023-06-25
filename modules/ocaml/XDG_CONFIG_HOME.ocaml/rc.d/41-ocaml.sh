#!/bin/sh

# shellcheck disable=SC1091
[ ! -r "${OPAMROOT}/opam-init/init.zsh" ] || . "${OPAMROOT}/opam-init/init.zsh"  > /dev/null 2> /dev/null
