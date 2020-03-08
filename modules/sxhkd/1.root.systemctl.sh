#!/bin/sh

invoker_home=$(getent passwd "${SUDO_USER-$USER}" | cut -d: -f6)

systemctl enable --user --now sxhkd
