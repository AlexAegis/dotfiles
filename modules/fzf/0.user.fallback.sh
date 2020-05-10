#!/bin/sh

git clone --depth 1 https://github.com/junegunn/fzf.git "$XDG_DATA_HOME/fzf"
"$XDG_DATA_HOME/fzf/install" --bin

cd "$XDG_CONFIG_HOME/environment.d" || exit 1
ln -sf "$PONT_MODULES_HOME/fzf/resources/51-fzf.conf" 51-fzf.conf
