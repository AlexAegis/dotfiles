#!/bin/sh

systemctl enable --user --now polybar.service
systemctl enable --user --now polybar-config.path
