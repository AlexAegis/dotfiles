#!/bin/sh

cp ./resources/nginx/nginx.pihole.conf "$NGINX_SITES_ENABLED/"
cp ./resources/systemd/resolved.conf /etc/systemd
cp ./resources/systemd/pihole.conf /etc/systemd/system/php-fpm.service.d/

reload_service php-fpm
reload_service systemd-resolved
reload_service pihole-FTL
reload_service nginx

# Add custom blocklists
if timelock pihole_blacklist_sync; then
	curl https://gist.githubusercontent.com/anudeepND/adac7982307fec6ee23605e281a57f1a/raw/5b8582b906a9497624c3f3187a49ebc23a9cf2fb/Test.txt | xargs -n 100 pihole -b
	curl https://raw.githubusercontent.com/Ewpratten/youtube_ad_blocklist/master/blocklist.txt | xargs -n 100 pihole -b
	curl https://blocklistproject.github.io/Lists/ads.txt | sed -e 's/\( *\)#.*$/\1/' | cut -d ' ' -f 2 | xargs -n 100 pihole -b
	echo "Locking Sync for a week using the pihole_blacklist_sync lock"
	timelock pihole_blacklist_sync 'week'
else
	echo "The pihole_blacklist_sync timelock is locked,\
 it unlocks at $(timelock -p pihole_blacklist_sync)"
fi
