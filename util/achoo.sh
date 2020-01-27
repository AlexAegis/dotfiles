#!/bin/bash

# test
function achoo {
	echo "$@"
}

if [ "${BASH_SOURCE[0]}" -ef "$0" ]
then
    achoo "$@"
fi
