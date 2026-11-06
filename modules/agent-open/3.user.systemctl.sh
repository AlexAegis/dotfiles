#!/bin/sh

systemctl --user daemon-reload || exit 1
systemctl --user reset-failed agent-open.socket
systemctl --user enable agent-open.socket || exit 1

# Outside a desktop session, the socket starts at the next login
systemctl --user is-active -q graphical-session.target || exit 0
systemctl --user restart agent-open.socket
