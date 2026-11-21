#!/bin/sh

systemctl --user disable --now approval-queue.socket
systemctl --user stop 'approval-queue@*.service'
