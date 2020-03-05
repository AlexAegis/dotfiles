#!/bin/fish
# source in folder
# set -a TODO: fishify if needed
for f in (find "$1" -maxdepth 1 -regex $2)
	echo Sourcing $f;
	. $f
end
# set +a
