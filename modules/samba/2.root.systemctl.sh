#!/bin/sh

systemctl enable --now samba.service
systemctl enable --now nmb.service
