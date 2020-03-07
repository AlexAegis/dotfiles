#!/bin/sh

systemctl --user daemon-reload
systemctl --user enable --now imwheel
journalctl --user --unit imwheel
