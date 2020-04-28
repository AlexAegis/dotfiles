#!/bin/sh

apt install -y python python-pip


# from https://github.com/pyenv/pyenv/wiki/common-build-problems
apt install -y make build-essential libssl-dev zlib1g-dev libbz2-dev \
	libreadline-dev libsqlite3-dev wget curl llvm libncurses5-dev \
	libncursesw5-dev xz-utils tk-dev libffi-dev liblzma-dev python-openssl git
