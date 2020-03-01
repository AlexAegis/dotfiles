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
# which you can use without dot itself.
# Dots other functionality is dependency resolvement. Dotmodules can
# depend on other dotmodules and it's dots job to install those beforehand
# Dot is also capable of skipping installations if nothing is changed
# since the last one. This is done by hashing the tar of the module.
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
# opens up a whiptail list of all the presets, selecting one opens it
# like dot -c preset p

# dot -mp PATH
# sets the m and p configs both to PATH which is the folders where the
# modules and presets live. It can be `;` separated.

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
# git submodule init
# git submodule update

show_help() {
	echo "install <modules>"
}

is_installed() {
	command -v "$1" 2>/dev/null
}

# Reset all variables that might be set
modules_selected=
config=0
# preset=
force=0
modules_folder="$HOME/.dotfiles/modules" # TODO: use relative to script
hashfilename=".tarhash"
dependenciesfilename=".dependencies"
tagsfilename=".tags"
verbose=0 # Print more
dry=0     # When set, no installation will be done

# Package manager availablity
has_pacman=$(is_installed pacman)
has_apt=$(is_installed apt)

# echo has_pacman $has_pacman
# echo has_apt $has_apt

while :; do
	# echo "Evaluating $1"
	case $1 in
	-h | -\? | --help) # Show help.
		show_help
		exit
		;;
	-v | --verbose) # Verbose printing TODO: Pass to the dotmodules
		verbose=1
		;;
	-c | --config) # Interactive customization installable modules
		config=1
		;;
	-d | --dry) #  customize installable modules
		dry=1
		;;
	-f | --force) # force installation
		force=1
		;;
	-p | preset) # Takes an option argument, ensuring it has been specified.
		if [ -n "$2" ]; then
			shift
			modules_selected=$*
		else
			modules_selected=$(whiptail --title "Select" \
				--checklist "Space changes selection, enter approves" \
				0 0 0 zsh zsh ON fish fish OFF 3>&1 1>&2 2>&3 3>&-)
		fi
		;;
	i | install) # Installs modules specified after it
		if [ -n "$2" ]; then
			shift
			modules_selected=$*
		else
			modules_selected=$(whiptail --title "Select" \
				--checklist "Space changes selection, enter approves" \
				0 0 0 zsh zsh ON fish fish OFF 3>&1 1>&2 2>&3 3>&-)
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

if [ -z "$modules_selected" ]; then
	echo 'Nothing to install'
	exit
fi

has_tag() {
	# Returns every dotmodule that contains a specific tag
	IFS='
	'
	# shellcheck disable=SC2016
	grep -lRm 1 -- "$@" "$modules_folder"/*/"$tagsfilename" |
		sed -r 's_^.*/([^/]*)/[^/]*$_\1_g'
}

get_dependencies() {
	if [ -f "$modules_folder/$1/$dependenciesfilename" ]; then
		deps_and_tags=$(sed -e 's/#.*$//' -e '/^$/d' \
			"$modules_folder/$1/$dependenciesfilename")
		deps=$(echo "$deps_and_tags" | sed -e 's/:.*$//')
		echo "$deps"
		tags=$(echo "$deps_and_tags" | sed -n '/:/p' | sed -e 's/://')
		# shellcheck disable=SC2086
		has_tag $tags
	fi
}

already_installed=""

install_dependencies() {
	while :; do
		if [ "$1" ]; then
			[ $verbose = 1 ] && echo "    Trying to install $1..."
			if [ "$(echo "$already_installed" | grep -w "$1")" = "" ]; then
				# shellcheck disable=SC2046
				install_dependencies $(get_dependencies "$1")
				install_module "$1"
				already_installed="$already_installed $1"
				[ $verbose = 1 ] && echo "    done."
			else
				[ $verbose = 1 ] && echo "    Already resolved."
			fi
			shift
		else
			break
		fi
	done
}

install_module() {

	module=$1
	shift

	if [ ! -d "$modules_folder/$module/" ]; then
		echo "Module $module not found. Skipping"
		return 1
	fi

	# cd to dotmodule just in case a dotmodule
	# is not suited for installation outside of it
	cd "${0%/*}" || return
	echo "Installing $module"

	# Only calculate the hashes if we going to use it
	if [ "$force" = 0 ]; then
		old_hash=$(cat "$modules_folder/$module/$hashfilename" 2>/dev/null)
		new_hash=$(tar --absolute-names \
			--exclude="$modules_folder/$module/$hashfilename" \
			-c "$modules_folder/$module" | sha1sum)
		[ $verbose = 1 ] && printf "%s\n%s" "$old_hash" "$new_hash"
	fi

	if
		[ "$force" = 1 ] || [ "$old_hash" != "$new_hash" ]
	then

		if [ "$has_apt" ] &&
			[ -f "$modules_folder/$module/install.apt.sh" ]; then
			# shellcheck disable=SC2091
			[ $dry != 1 ] && $("$modules_folder/$module/install.apt.sh")
		fi
		if [ "$has_pacman" ] &&
			[ -f "$modules_folder/$module/install.pacman.sh" ]; then
			# shellcheck disable=SC2091
			[ $dry != 1 ] && $("$modules_folder/$module/install.pacman.sh")
		fi
		if [ -f "$modules_folder/$module/install.sudo.sh" ]; then
			# shellcheck disable=SC2091
			[ $dry != 1 ] && $("$modules_folder/$module/install.sudo.sh")
		fi
		if [ -f "$modules_folder/$module/install.sh" ]; then
			# shellcheck disable=SC2091
			[ $dry != 1 ] && $("$modules_folder/$module/install.sh")
		fi

		# Calculate fresh hash (always)
		[ $dry != 1 ] &&
			tar --absolute-names \
				--exclude="$modules_folder/$module/$hashfilename" \
				-c "$modules_folder/$module" |
			sha1sum >"$modules_folder/$module/$hashfilename"
	else
		echo "$1 is already installed and no changes are detected"
	fi

}

# Actual installation process
# This will install a non-existent module that depends on the selected modules
# shellcheck disable=SC2086
install_dependencies $modules_selected
