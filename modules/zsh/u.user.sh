#!/bin/sh

echo "Updating plugins"

# if it has problems regarding changes in the cache just delete it and reinstall
# the module rm -rf ~/.cache/antibody/                                                                                                               ✔  23:05:50
rm "$ZDOTDIR"/.zcompdump*
"$ZDOTDIR/bin/antibody" update
