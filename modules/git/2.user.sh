#!/bin/sh

# Create initial user.gitconfig file
# Since this can be different from system to system I only create a default
# one that is free to change without changing the global config

if ! [ -e "$XDG_CONFIG_HOME/git/gitconfig.d/user.gitconfig" ]; then
cat <<EOF > "$XDG_CONFIG_HOME/git/gitconfig.d/user.gitconfig"
[user]
	name = AlexAegis
	email = alexaegis@pm.me

[github]
	user = AlexAegis
EOF
fi
