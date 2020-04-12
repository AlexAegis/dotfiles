#!/bin/sh

systemctl enable --user --now polybar@main:1.service
systemctl enable --user --now polybar@main:2.service
systemctl enable --user --now polybar-watcher@main:1.service
systemctl enable --user --now polybar-watcher@main:2.service
