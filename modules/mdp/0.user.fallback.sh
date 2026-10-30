#!/bin/sh

[ -d "$XDG_CACHE_HOME/mdp/.git" ] \
	|| git clone https://github.com/visit1985/mdp.git "$XDG_CACHE_HOME/mdp" \
	|| exit 1
cd "$XDG_CACHE_HOME/mdp" || exit 1
make
make install
