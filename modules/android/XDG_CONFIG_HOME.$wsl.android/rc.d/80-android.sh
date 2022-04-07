#!/bin/sh

WSL_HOST="$(tail -1 /etc/resolv.conf | cut -d' ' -f2)"
export WSL_HOST
export ADB_SERVER_SOCKET=tcp:"$WSL_HOST":5037
