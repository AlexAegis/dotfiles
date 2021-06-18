#!/bin/sh

# Dynammically assembling a .profile file that can serve as a standalone
# entry point for shells where the initial environment is not necessarily
# available.

echo "#!/bin/sh
# POSIX compliant defaults sourced from every other shell
#
# Load environmental variables and aliases
set -a
# shellcheck disable=SC1090,SC1091
if [ -e \"$XDG_BIN_HOME/loadenv\" ]; then
	. \"$XDG_BIN_HOME/loadenv\"
fi
set +a
" > "$HOME/.profile"
