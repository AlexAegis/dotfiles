#!/bin/sh

mkdir -p /etc/NetworkManager/conf.d

cat <<EOF > /etc/NetworkManager/conf.d/dns.conf
[main]
dns=systemd-resolved
EOF

mkdir -p /etc/systemd/resolved.conf.d/

cat <<EOF > /etc/systemd/resolved.conf.d/dns_servers.conf
[Resolve]
DNS=192.168.0.1
Domains=~.
EOF

cat <<EOF > /etc/systemd/resolved.conf.d/fallback_dns.conf
[Resolve]
FallbackDNS=1.1.1.1 127.0.0.1 ::1
EOF


systemctl enable --now systemd-resolved.service
