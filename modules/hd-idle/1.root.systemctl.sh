#!/bin/sh

crudini --merge /etc/default/hd-idle << EOF
START_HD_IDLE=true
EOF

systemctl enable --now hd-idle.service
