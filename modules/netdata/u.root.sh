#!/bin/sh

# Enable site
echo $NGINX_SITES_ENABLED
cp ./resources/site-netdata.conf "$NGINX_SITES_ENABLED/"
reload_service nginx
