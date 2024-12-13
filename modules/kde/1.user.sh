#!/bin/sh

# On KDE let it ask the pass for git
git config --global core.askpass /usr/bin/ksshaskpass

# crudini --merge "$XDG_CONFIG_HOME/kwinrc" << EOF
# [Compositor]
# MaxFPS=144
# EOF
