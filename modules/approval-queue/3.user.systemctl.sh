#!/bin/sh

systemctl --user daemon-reload || exit 1
# Clears a start limit that earlier failed starts reached. At the first install,
# systemd has not loaded the unit yet and reports it.
systemctl --user reset-failed approval-queue.socket 2>/dev/null
systemctl --user enable approval-queue.socket || exit 1
# The service processes of the waiting prompts keep their connections
systemctl --user restart approval-queue.socket
