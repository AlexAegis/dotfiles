#!/bin/sh

cargo install aura-pm


# On ARM, clone aura, checkout v3.2.9 (4.0.0-rc1) as Manjaro ARM only has pacman 6 now
# libalpm_version=$(pacman --version | grep -oP 'libalpm v\K[0-9]+(?:\.[0-9]+)*')

# git clone https://github.com/fosskers/aura
# git checkout v3.2.9
# cargo install --path rust/aura --features "alpm/generate"
