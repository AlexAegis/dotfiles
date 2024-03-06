#!/bin/sh

# Since I can't have variables in a `.desktop` entry, instead of symlinking
# it I have to create it with the $HOME variable of the installer
echo "\
[Desktop Entry]
Type=Application
Name=Capture Desktop Environment
Comment=Save the current desktop environment to ~/desktop-environment.txt
Icon=utilities-terminal
Exec=sh -c \"printenv > $HOME/desktop-environment.txt\"
" > "$XDG_DATA_HOME/applications/capture-desktop-env.desktop"
