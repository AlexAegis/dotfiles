#!/bin/sh

systemctl --user disable --now agent-wayland-proxy.socket
# Also stops the proxy of each client
systemctl --user stop agent-wayland-context.service
