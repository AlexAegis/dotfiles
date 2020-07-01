#!/bin/zsh

if [ "$TERM" = "linux" ]; then
	echo -en "\e]P0151515" # Black
	echo -en "\e]P1ac4142" # Red
	echo -en "\e]PEac4142" # Bright Cyan
	echo -en "\e]PFac4142" # Bright White
	clear                  # Clear artifacts
fi

# Welcome
command -v neofetch > /dev/null && neofetch
