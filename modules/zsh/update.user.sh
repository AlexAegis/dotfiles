#!/bin/sh

# if it has problems regarding changes in the cache just delete it and reinstall
# the module rm -rf ~/.cache/antibody/                                                                                                               ✔  23:05:50
echo "Updating plugins"
rm "$ZDOTDIR"/.zcompdump*
antibody update
