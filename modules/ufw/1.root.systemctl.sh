#!/bin/sh

ufw enable
systemctl enable --now ufw.service

