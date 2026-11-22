#!/bin/sh

# Its stop unloads the KWin script
systemctl --user disable --now approval-queue-notify.service
systemctl --user disable --now approval-queue.socket
systemctl --user stop 'approval-queue@*.service'
