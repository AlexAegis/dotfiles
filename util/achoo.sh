#!/bin/bash

# Only tries to installs if package is not present
function achoo {
	echo "$@"
}

if [ "${BASH_SOURCE[0]}" -ef "$0" ]
then
    achoo "$@"
fi
