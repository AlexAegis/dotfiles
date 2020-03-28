#!/bin/sh

echo 'starting and enabling timesyncd service'
systemctl enable --now xdg-runtime-cleanup.service
systemctl enable --now xdg-runtime-reset.service
