#!/bin/sh

systemctl --user daemon-reload || exit 1
# Clears a start limit that earlier failed starts reached. At the first install,
# systemd has not loaded the unit yet and reports it.
systemctl --user reset-failed agent-open.socket 2>/dev/null
systemctl --user enable agent-open.socket || exit 1

# Outside a desktop session, the socket starts at the next login
systemctl --user is-active -q graphical-session.target || exit 0
systemctl --user restart agent-open.socket
