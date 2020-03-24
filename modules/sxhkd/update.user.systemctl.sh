#!/bin/sh

systemctl --user daemon-reload
systemctl restart --user sxhkd
systemctl restart --user kill_sxhkd_on_change
