#!/bin/sh

# Turn off pipewire-media-session if running
if systemctl --user is-enabled pipewire-media-session; then
	systemctl --user --now disable pipewire-media-session
fi

if systemctl --user is-enabled pulseaudio.socket; then
	systemctl --user --now disable pulseaudio.socket
fi

if systemctl --user is-enabled pulseaudio.service; then
	systemctl --user --now disable pulseaudio.service
fi

# enable pipewire
systemctl --user --now enable pipewire
systemctl --user --now enable pipewire-pulse
systemctl --user --now enable wireplumber

