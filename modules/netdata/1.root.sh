#!/bin/sh

# Opt out telemetry
touch /etc/netdata/.opt-out-from-anonymous-statistics

# Disable cloud functionality
NETDATA_CLOUD_D="/var/lib/netdata/cloud.d"
mkdir -p "$NETDATA_CLOUD_D"
cat << EOF > "$NETDATA_CLOUD_D/cloud.conf"
[global]
    enabled = no
EOF

chmod 0770 "$NETDATA_CLOUD_D/cloud.conf"
chown netdata:netdata "$NETDATA_CLOUD_D/cloud.conf"

