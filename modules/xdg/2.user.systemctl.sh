#!/bin/sh

# Applies user-tmpfiles.d at each start of the user manager
systemctl --user enable systemd-tmpfiles-setup.service
