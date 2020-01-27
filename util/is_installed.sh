#!/bin/bash

function is_installed {
	if [ -f /usr/bin/apt ]; then
		result=$(dpkg -s $1 2>/dev/null | grep ^Status )
	elif [ -f /usr/bin/rpm ]; then
		result=$(rpm -qa | grep ^tar)
	fi
	echo $result
}

if [ "${BASH_SOURCE[0]}" -ef "$0" ]
then
    is_installed "$@"
fi
