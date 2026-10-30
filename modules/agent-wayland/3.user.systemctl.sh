#!/bin/sh

systemctl --user daemon-reload || exit 1
# Clears a start limit that earlier failed starts reached
systemctl --user reset-failed agent-wayland-proxy.socket agent-wayland-context.service
systemctl --user enable agent-wayland-proxy.socket || exit 1

# Outside a desktop session, the services have no WAYLAND_DISPLAY. The session
# starts the socket at the next login.
systemctl --user is-active -q graphical-session.target || exit 0
# Running units keep their old settings and binaries until they restart, which
# closes agent windows
systemctl --user restart agent-wayland-proxy.socket || exit 1
systemctl --user try-restart agent-wayland-context.service
