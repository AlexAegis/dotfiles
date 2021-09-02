#!/bin/sh

pacman -Syu --needed --noconfirm php \
	php-fpm \
	php-cgi \
	php-sqlite \
	php-intl
