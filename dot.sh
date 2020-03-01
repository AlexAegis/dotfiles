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

C_RESET='\033[0m'
C_BLUE='\033[0;34m'
C_CYAN='\033[0;36m'
C_RED='\033[0;31m'
C_WHITE='\033[0;37m'
C_YELLOW='\033[0;33m'

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
presets_folder="$HOME/.dotfiles/presets" # TODO: use relative to script
preset_extension=".preset"
hashfilename=".tarhash"
dependenciesfilename=".dependencies"
tagsfilename=".tags"
verbose=0 # Print more
dry=0     # When set, no installation will be done

all_modules=$(find "$modules_folder/" -maxdepth 1 -mindepth 1 -printf "%f\n")
all_presets=$(find "$presets_folder/" -maxdepth 1 -mindepth 1 \
	-name '*.preset' -printf "%f\n" | sed 's/.preset//')

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
	-p | --preset) # Takes an option argument, ensuring it has been specified.
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

[ $verbose = 1 ] && echo "$all_modules"
[ $verbose = 1 ] && echo "$all_presets"

# Package manager availablity
has_pacman=$(is_installed pacman)
has_apt=$(is_installed apt)

[ $verbose = 1 ] && echo has_pacman "$has_pacman"
[ $verbose = 1 ] && echo has_apt "$has_apt"

if [ -z "$modules_selected" ]; then
	echo 'Nothing to install'
	exit
fi

has_tag() {
	# Returns every dotmodule that contains any of the tags
	IFS='
	'
	# shellcheck disable=SC2016
	grep -lRm 1 -- "$@" "$modules_folder"/*/"$tagsfilename" |
		sed -r 's_^.*/([^/]*)/[^/]*$_\1_g'
}

in_preset() {
	# returns every
	if [ -f "$presets_folder/$1$preset_extension" ]; then
		sed -e 's/#.*$//' -e '/^$/d' "$presets_folder/$1$preset_extension"
	fi
}

get_dependencies() {
	if [ -f "$modules_folder/$1/$dependenciesfilename" ]; then
		sed -e 's/#.*$//' -e '/^$/d' "$modules_folder/$1/$dependenciesfilename"
	fi
}

resolved=""

install_whatever() {
	while :; do
		if [ "$1" ]; then
			[ $verbose = 1 ] && echo "    Trying to install $1..."
			if [ "$(echo "$resolved" | grep -w "$1")" = "" ]; then
				resolved="$resolved $1"
				case "$1" in
				+*) # presets
					# shellcheck disable=SC2046
					install_whatever $(in_preset "$(echo "$1" | cut -c2-)")
					;;
				:*) # tags
					# shellcheck disable=SC2046
					install_whatever $(has_tag "$(echo "$1" | cut -c2-)")
					;;
				*) # modules
					# shellcheck disable=SC2046
					install_whatever $(get_dependencies "$1")
					install_module "$1"
					;;
				esac
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
	echo "${C_BLUE}Installing $module$C_RESET"

	# Only calculate the hashes if we going to use it
	if [ "$force" = 0 ]; then
		old_hash=$(cat "$modules_folder/$module/$hashfilename" 2>/dev/null)
		new_hash=$(tar --absolute-names \
			--exclude="$modules_folder/$module/$hashfilename" \
			-c "$modules_folder/$module" | sha1sum)
		[ $verbose = 1 ] && printf "$C_RED%s\n%s\n$C_RESET" "$old_hash" "$new_hash"
	fi

	if
		[ "$force" = 1 ] || [ "$old_hash" != "$new_hash" ]
	then
		echo "${C_RED}Applying dotmodule $module$C_RESET"
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

		# Calculate fresh hash (always if not dryrunning)
		[ $dry != 1 ] &&
			tar --absolute-names \
				--exclude="$modules_folder/$module/$hashfilename" \
				-c "$modules_folder/$module" |
			sha1sum >"$modules_folder/$module/$hashfilename"
	else
		echo "$C_YELLOW! $module is already installed and no changes are detected$C_RESET"
	fi

}

# Actual installation process
# This will install a non-existent module that depends on the selected modules
# shellcheck disable=SC2086
install_whatever $modules_selected
