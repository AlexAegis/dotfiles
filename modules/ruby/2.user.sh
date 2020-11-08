#!/bin/sh

latest_ruby_version=$(rbenv install --list 2>/dev/null | sed -n '/^[0-9].*$/h;$!b;g;p')

echo "Installing ruby $latest_ruby_version using rbenv"

rbenv install --skip-existing "$latest_ruby_version"

rbenv global $latest_ruby_version
