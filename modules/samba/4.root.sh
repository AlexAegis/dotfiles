#!/bin/sh

# enable sambe through the firewall from the local network
echo "
[Samba]
title=Samba
description=Samba as domain controller
ports=53|88|135/tcp|137/udp|138/udp|139/tcp|389|445/tcp|464|636/tcp|1024:5000/tcp|3268/tcp|3269/tcp|5353
" > /etc/ufw/applications.d/samba

ufw allow from 192.168.0.0/16 to any app samba
