#!/bin/sh

# Enable site
cp ./resources/nginx.netdata.conf "$NGINX_SITES_ENABLED/"
reload_service nginx
