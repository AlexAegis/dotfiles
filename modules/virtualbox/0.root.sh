#!/bin/sh

# Load dynamic kernel modules
dkms autoinstall
modprobe vboxdrv
