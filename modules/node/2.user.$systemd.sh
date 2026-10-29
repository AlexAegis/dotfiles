#!/bin/sh

exec systemd-tmpfiles --user --create ./XDG_CONFIG_HOME.node/user-tmpfiles.d/pnpm.conf
