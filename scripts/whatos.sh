#!/bin/bash
# Reads the NAME of the OS.
function whatos {
	cat /etc/os-release | grep "^NAME" | grep -oh "\".*\"" | tail -c +2 | head -c -2
}

# Execute if not sourced
if [ "${BASH_SOURCE[0]}" -ef "$0" ]
then
    whatos
fi
