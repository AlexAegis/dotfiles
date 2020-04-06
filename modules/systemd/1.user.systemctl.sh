#!/bin/sh

systemctl enable --user --now user-service-watcher.service
# Don't put --now here, can't enable without instance name.
# And they will be only started by other services
systemctl enable --user daemon-reload.service
systemctl enable --user notify-low@.service
systemctl enable --user notify-normal@.service
systemctl enable --user notify-critical@.service
systemctl enable --user notify-failure@.service
systemctl enable --user notify-restart@.service
