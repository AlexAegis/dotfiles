#!/bin/sh

systemctl enable --user --now daemon-reload.service
systemctl enable --user --now user-services.path
