#!/bin/sh

echo 'starting and enabling xdg-runtime services'
systemctl enable --now xdg-runtime-cleanup.service
systemctl enable --now xdg-runtime-reset.service
