#!/bin/sh

# the XDG setup needs to be loaded manually during startup, the first file
# zsh laods under our control is ~/.zshenv here, I tell it where it can find
# the zshrc file. But both are located in XDG_CONFIG_HOME. That variable might
# not be available when .zshrc is loaded so a concrete path is needed to be
# set in that file. Since `pont` knows these variables by the time it gets
# here, I can just dynamically create the `.zshenv` file


echo "#!/bin/zsh
# This is here for compatibility reasons, using a login manager, this is not
# needed, but when logging directly into zsh, systemd envs are not available

# ZSH Environment
if [ -e \"$XDG_CONFIG_HOME/environment.d/20-zshenv.conf\" ]; then
	. \"$XDG_CONFIG_HOME/environment.d/20-zshenv.conf\"
fi
# rest is loaded by the startup scripts in ZDOTDIR, set in 20-zshenv.conf
# inluding the ~/.profile file (which in turn loads the entire environment)
" > "$HOME/.zshenv"
