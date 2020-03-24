#!/bin/sh

systemctl enable --user --now sxhkd
systemctl enable --user --now kill_sxhkd_on_change
