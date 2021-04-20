#!/bin/sh

"$XDG_BIN_HOME/install-portage-config.sh"

# Other Overlays might call it like this
# emerge -u app-editors/visual-studio-code

emaint add jorgicio
emaint sync -r jorgicio
emerge -u app-editors/vscode-bin

