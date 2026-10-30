#!/bin/sh

# Group agent enters the folder of the socket. The owner runs the proxy.
owner=${SUDO_USER:?pont sets SUDO_USER for root steps}
conf=/etc/tmpfiles.d/agent-wayland.conf
printf 'd /run/agent-wayland 2750 %s agent -\n' "$owner" >"$conf" || exit 1
chown root:root "$conf" || exit 1
chmod 644 "$conf" || exit 1
systemd-tmpfiles --create "$conf"
