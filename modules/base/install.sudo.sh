#!/bin/sh

# Making an absolute path form a relative one

DOT_PATH=$(
	cd "${0%/*}/../.."
	pwd
)

# Using a symlink to make dot available

ln -sf $DOT_PATH/dot /usr/local/bin/dot
