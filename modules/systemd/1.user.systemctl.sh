#!/bin/sh

systemctl enable --user --now user-service-watcher.service
systemctl enable --user --now environment-watcher.service
# Don't put --now here, can't enable without instance name.
# And they will be only started by other services
systemctl enable --user daemon-reload.service
