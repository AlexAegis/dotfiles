#!/bin/sh

echo 'Reload sxhkd'
notify-send 'Reload Sxkhd'
/usr/bin/systemctl restart --user sxhkd.service
