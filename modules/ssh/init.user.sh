#!/bin/sh

# If restored the `~/.ssh` folder, then not removing those
# files that are present in the module and in the .ssh folder
# would cause conflicts.
rm "$HOME/.ssh/config"
