#!/bin/sh

npm install -g turbo

turbo completion zsh > "$XDG_CONFIG_HOME/zsh/compdef/_turbo"
# TODO: Remove this once https://github.com/vercel/turbo/issues/3802 is resolved, this makes the competion half broken. But it's better than fully broken
sed -i 's/\*\::/::/g' "$XDG_CONFIG_HOME/zsh/compdef/_turbo"
