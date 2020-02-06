#!/bin/bash

mkdir -p $HOME/.env # Don't let stow manage shared folders
stow -Rt ~ home
