#!/bin/bash

source ./install-required.sh

# Pull and update submodules
git submodule init
git submodule update

choices=$(whiptail\
	--title "Select"\
	--checklist "Space changes selection, enter approves" 0 0 0\
	zsh zsh ON\
	fish fish OFF\
	3>&1 1>&2 2>&3 3>&-)

echo Choices:
echo $choices

for choice in $choices
do
	echo "Installing $choice"
	source "./programs/$choice/install.sh"
done
