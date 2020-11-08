#!/bin/sh

# Update rbenv
cd "$RBENV_ROOT" || exit 1
git pull

# Update plugins
if [ -d "$RBENV_ROOT/plugins/ruby-build/.git" ]; then
	cd "$RBENV_ROOT/plugins/ruby-build" || exit 1
	git pull
fi
