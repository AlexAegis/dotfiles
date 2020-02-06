#!/bin/bash
# Dependencies: yay

yay -Sya snapd
sudo systemctl enable snapd
sudo systemctl start snapd

# TODO Add to path /var/lib/snapd/snap/bin

PATH="$PATH:/var/lib/snapd/snap/bin"
