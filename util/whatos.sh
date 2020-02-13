#!/bin/sh
# Reads the NAME of the OS. Does not work on Arch
function whatos {
	cat /etc/os-release | grep "^NAME" | grep -oh "=.*" | tr -d '="'
}

# Execute if not sourced
if [ "${BASH_SOURCE[0]}" -ef "$0" ]
then
    whatos
fi
