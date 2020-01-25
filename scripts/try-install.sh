#!/bin/bash

# Only tries to installs if package is not present
function try_install {
	if [[ $(dpkg -s $1 | grep Status ) == Status* ]] ;
	then
		echo "$1 is already installed"
	else
		apt-get -y install $1 &&\
			echo "$1 just got installed" ||\
			echo "$1 couldn't be installed"
	fi
}

if [ "${BASH_SOURCE[0]}" -ef "$0" ]
then
    try_install $1
fi
