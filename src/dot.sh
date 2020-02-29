#!/bin/sh

#      _       _
#   __| | ___ | |_
#  / _` |/ _ \| __|
# | (_| | (_) | |_
#  \__,_|\___/ \__|
#
# A simple dotmodule manager
#
# Dots main purpose is to invoke scripts defined in dotmodules
# It is designed this way so each dotmodule is a self contained entity
# which you can use without dot itself
# Dots other functionality is dependency resolvement. Dotmodules can
# depend on other dotmodules and it's dots job to install those beforehand
# Dot is also capable of skipping installations if nothing is changed since the last one
# This is done by hashing the tar of the module.
#
# Each dotmodule is unique and the only common part of each of them is
# that you have to stow a folder to your home directory.
# But besides that you might have to install packages too.
# And packages are too can be installed differently on different systems
#
#

# dot install
# sets config 1 preset 0, opens up whiptail list without selections

# dot install a b
# installs module a b

# dot -c install a b
# opens up whiptail with a and b preselected

# dot preset p
# installs preset p

# dot -c preset p
# opens up whiptail with all the modules of p selected

# dot preset
# opens up a whiptail list of all the presets, selecting one installs it

# dot -c preset
# opens up a whiptail list of all the presets, selecting one opens it like dot -c preset p

# dot -mp PATH
# sets the m and p configs both to PATH which is the folders where the modules and presets live
# may be ; separated

# dot
# dot help
# dot -h
# dot --help
# opens up help

# dot -f --force
# by default dot compares a tarhash of the last installed module, and only reinstalls if it changed
# if forced however it could skip this test.
# It always updates the hash regardless of forcing or not
# dont forget to ignore the hashfile itself from the hashing

# Pull and update submodules
git submodule init
git submodule update

# if [ "${BASH_SOURCE[0]}" -ef "$0" ]
# then
#
# 	choices=$(whiptail\
# 		--title "Select"\
# 		--checklist "Space changes selection, enter approves" 0 0 0\
# 		zsh zsh ON\
# 		fish fish OFF\
# 		3>&1 1>&2 2>&3 3>&-)
#
# 	for choice in $choices
# 	do
# 		choice=$(echo $choice | xargs) # removing quotemarks
# 		echo "Installing $choice"
# 		source "./programs/$choice/install.sh"
# 	done
# fi

show_help() {
	echo "install <modules>"
}

# Reset all variables that might be set
modules_selected=
config=0
preset=
force=0
modules_folder=$HOME/.dotfiles/modules
hashfilename=".tarhash"
verbose=0 # Variables to be evaluated as shell arithmetic should be initialized to a default or validated beforehand.

# Package manager availablity
has_pacman=$(is_installed pacman)
has_apt=$(is_installed apt)
echo has_pacman $has_pacman
echo has_apt $has_apt

while :; do
	case $1 in
	-h | -\? | --help) # Call a "show_help" function to display a synopsis, then exit.
		show_help
		exit
		;;
	-c | --config) # Makes it so a whiptail config will appear to customize installable modules
		config=1
		shift
		;;
	-p | preset) # Takes an option argument, ensuring it has been specified.
		if [ -n "$2" ]; then
			shift
			modules_selected=$*
		else
			modules_selected=$(whiptail --title "Select" \
				--checklist "Space changes selection, enter approves" 0 0 0 zsh zsh ON fish fish OFF 3>&1 1>&2 2>&3 3>&-)
		fi
		;;
	i | install) # Installs modules specified after it
		if [ -n "$2" ]; then
			shift
			modules_selected=$*
		else
			modules_selected=$(whiptail --title "Select" \
				--checklist "Space changes selection, enter approves" 0 0 0 zsh zsh ON fish fish OFF 3>&1 1>&2 2>&3 3>&-)
		fi
		;;
	--) # End of all options.
		shift
		break
		;;
	-?*)
		printf 'WARN: Unknown option (ignored): %s\n' "$1" >&2
		;;
	*) # Default case: If no more options then break out of the loop.
		break ;;
	esac

	shift
done

if [ -n "$modules_selected" ]; then
	echo "$modules_selected"
else
	echo 'Nothing to install'
fi

install() {
	# Only calculate the hashes if we going to use it
	if [ "$force" = 0 ]; then
		old_hash=$(cat "$modules_folder/$1/$hashfilename" >/dev/null 2>&1 &)
		new_hash=$(tar --absolute-names --exclude="$modules_folder/$1/$hashfilename" -c "$modules_folder/$1" | sha1sum)
	fi

	if
		[ "$force" = 1 ] || [ "$old_hash" != "$new_hash" ]
	then
		echo Installing
		if [ $has_apt ] && [ -f "$modules_folder/$1/install.apt.sh" ]; then
			$("$modules_folder/$1/install.apt.sh")
		fi
		if [ $has_pacman ] && [ -f "$modules_folder/$1/install.pacman.sh" ]; then
			$("$modules_folder/$1/install.pacman.sh")
		fi
		if [ -f "$modules_folder/$1/install.sudo.sh" ]; then
			$("$modules_folder/$1/install.sudo.sh")
		fi
		if [ -f "$modules_folder/$1/install.sh" ]; then
			$("$modules_folder/$1/install.sh")
		fi

		# Calculate fresh hash (always)
		tar --absolute-names --exclude="$modules_folder/$1/$hashfilename" -c "$modules_folder/$1" | sha1sum >"$modules_folder/$1/$hashfilename"
	else
		echo "$1 already installed, no changes detected"
	fi

}

# Actual installation process
for module in $modules_selected; do
	cd "${0%/*}"
	echo Installing $module
	install $module
done
