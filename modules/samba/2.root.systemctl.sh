#!/bin/sh

systemctl enable --now smb.service
systemctl enable --now nmb.service
systemctl enable --now wsdd.service
