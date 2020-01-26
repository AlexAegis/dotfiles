#!/bin/bash

source ./
source ./install-required.sh

# Pull and update submodules
git submodule init
git submodule update


if [ "${BASH_SOURCE[0]}" -ef "$0" ]
then

	choices=$(whiptail\
		--title "Select"\
		--checklist "Space changes selection, enter approves" 0 0 0\
		zsh zsh ON\
		fish fish OFF\
		3>&1 1>&2 2>&3 3>&-)

	for choice in $choices
	do
		choice=$(echo $choice | xargs) # removing quotemarks
		echo "Installing $choice"
		source "./programs/$choice/install.sh"
	done


fi



