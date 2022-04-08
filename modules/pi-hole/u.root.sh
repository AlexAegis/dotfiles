#!/bin/sh

cp ./resources/nginx/nginx.pihole.conf "$NGINX_SITES_ENABLED/"
cp ./resources/nginx/nginx.pihole-proxy.conf "$NGINX_SITES_ENABLED/"
cp ./resources/systemd/resolved.conf /etc/systemd
cp ./resources/systemd/pihole.conf /etc/systemd/system/php-fpm.service.d/
cp ./resources/pihole-updatelists.conf /etc/
cp ./resources/pihole-FTL.conf /etc/pihole/pihole-FTL.conf

reload_service php-fpm
reload_service systemd-resolved
reload_service pihole-FTL
reload_service nginx

# This updates the updater itself
pihole-updatelists --update

# Update immediately
if command -v systemctl >/dev/null; then
	systemctl start pihole-updatelists.service
fi
