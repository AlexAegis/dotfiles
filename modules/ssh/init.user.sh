#!/bin/sh

# If restored the `~/.ssh` folder then not removing
# files that are present in the .ssh folder in the module
# would cause conflicts. And if it was
rm "$HOME/.ssh/config"
