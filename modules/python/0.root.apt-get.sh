#!/bin/sh

apt-get install -y python3 python3-pip

# from https://github.com/pyenv/pyenv/wiki/common-build-problems
apt-get install -y make build-essential libssl-dev zlib1g zlib1g-dev \
	libbz2-dev libreadline-dev libsqlite3-dev llvm libncurses5-dev \
	libncursesw5-dev xz-utils tk-dev libffi-dev liblzma-dev \
	gobjc++ python3-openssl
