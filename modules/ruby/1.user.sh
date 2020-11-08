#!/bin/sh

if [ -d "$RBENV_ROOT/.git" ]; then
	cd "$RBENV_ROOT" || exit 1
	git pull
else
	git clone https://github.com/rbenv/rbenv "$RBENV_ROOT"
fi



# Install plugins

if [ -d "$RBENV_ROOT/plugins/ruby-build/.git" ]; then
	cd "$RBENV_ROOT/plugins/ruby-build" || exit 1
	git pull
else
	git clone https://github.com/rbenv/ruby-build.git "$RBENV_ROOT/plugins/ruby-build"
fi
