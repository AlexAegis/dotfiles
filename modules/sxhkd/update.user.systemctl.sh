#!/bin/sh

systemctl --user daemon-reload
systemctl restart --user sxhkd
