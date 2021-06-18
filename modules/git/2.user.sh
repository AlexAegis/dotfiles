#!/bin/sh

# Create initial user.gitconfig file
# Since this can be different from system to system I only create a default
# one that is free to change without changing the global config

cat <<EOF > "$XDG_CONFIG_HOME/git/gitconfig.d/user.gitconfig"
[user]
	name = AlexAegis
	email = alexaegis@gmail.com

[github]
	user = AlexAegis
EOF
