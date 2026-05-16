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

# Create initial attributes file
# Since only one global attribute file can exist, instead of symlinking it
# we create a file and write into it from modules.
# TODO: Since this can leave garbage over time, a provider system should be implemented, where a provider script exports and extends a variable with it content
if ! [ -e "$XDG_CONFIG_HOME/git/attributes" ]; then
cat <<EOF > "$XDG_CONFIG_HOME/git/attributes"
EOF
fi
