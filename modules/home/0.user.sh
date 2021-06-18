#!/bin/sh

# Dynamically assembling a .profile file that can serve as a standalone
# entry point for shells where the initial environment is not necessarily
# available.

if [ -L "$HOME/.profile" ]; then
	rm "$HOME/.profile"
fi

if [ -e "$HOME/.profile" ] &&
	! grep -q "# THIS FILE WAS GENERATED BY A DOTMODULE" "$HOME/.profile"; then
	echo "existing ~/.profile file renamed to ~/.profile.bak"
	mv "$HOME/.profile" "$HOME/.profile.bak"
fi

# ? $XDG_CONFIG_HOME is intentionally evaluated in this file
cat <<EOF > "$HOME/.profile"
#!/bin/sh
# THIS FILE WAS GENERATED BY A DOTMODULE
# POSIX compliant defaults sourced from every other shell

# Load environmental variables and aliases
if [ -e "$XDG_BIN_HOME/loadenv" ]; then
	set -a
	# shellcheck disable=SC1090,SC1091
	. "$XDG_BIN_HOME/loadenv"
	set +a
fi
EOF
