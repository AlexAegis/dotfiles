#!/bin/sh

cp ./resources/qbittorrentd.service /etc/systemd/system/

systemctl enable --now qbittorrentd
