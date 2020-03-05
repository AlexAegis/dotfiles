#!/bin/sh

#      _       _
#   __| | ___ | |_
#  / _` |/ _ \| __|
# | (_| | (_) | |_
#  \__,_|\___/ \__|
#
# A simple dotmodule manager
#
# Run with `sudo -E dot` if you need root in some modules
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

# TODO: Uninstall by default unstow, if full uninstall then run the uninstall
# TODO: scripts, and uninstall should also remove the .tarhash file

# TODO: Clash support. Use .clash file, if two modules clash, ask which to use
# TODO: If a clashing module is already installed, abort, ask if interactive,
# remove other if forced
# TODO: ? optional dep resolve.

# TODO: Auto stow at start of install
# TODO: Auto unstow at end of uninstall

# TODO: .dot file in a dotmodule that is sourced before installing that module

# TODO: Fallback installation. Provide a $try_from_source when no packagemanager
# TODO: files are present (or when a .dot file is present and has this var set)
# TODO: See 'exa' module for example

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
# by default dot compares a tarhash of the last installed module, and only
# reinstalls if it changed
# if forced however it could skip this test.
# It always updates the hash regardless of forcing or not
# dont forget to ignore the hashfile itself from the hashing

# Pull and update submodules
# git submodule init
# git submodule update

C_RESET='\033[0m'
C_RED='\033[0;31m'
C_GREEN='\033[0;32m'
C_YELLOW='\033[0;33m'
C_BLUE='\033[0;34m'
C_CYAN='\033[0;36m'

show_help() {
	echo "install <modules>"
}

is_installed() {
	command -v "$1" 2>/dev/null
}

# Reset all variables that might be set
IFS='
'
modules_selected=""
resolved=""
final_module_list=""
config=0
force=0
fix_permissions=0
modules_folder="$HOME/.dotfiles/modules" # TODO: use relative to script
presets_folder="$HOME/.dotfiles/presets" # TODO: use relative to script
preset_extension=".preset"
hashfilename=".tarhash"
dependenciesfilename=".dependencies"
tagsfilename=".tags"
verbose=0 # Print more
dry=0     # When set, no installation will be done
# TODO: Only needed when printing and using whiptail. Lazy load it.
all_modules=$(find "$modules_folder/" -maxdepth 1 -mindepth 1 -printf "%f\n" |
	sort)
all_presets=$(find "$presets_folder/" -maxdepth 1 -mindepth 1 \
	-name '*.preset' -printf "%f\n" | sed 's/.preset//' | sort)
all_tags=$(find "$modules_folder"/*/ -maxdepth 1 -mindepth 1 -name '.tags' \
	-exec cat {} + | grep "^[^#;]" | sort | uniq)

# Package manager availablity
has_pacman=$(is_installed pacman)
has_apt=$(is_installed apt)

while :; do
	# echo "Evaluating $1"
	case $1 in
	-h | -\? | --help) # Show help.
		show_help
		exit
		;;
	-la | --list-all)
		printf "${C_BLUE}All available modules:${C_RESET}\n%s\n" "$all_modules"
		printf "${C_BLUE}All available presets:${C_RESET}\n%s\n" "$all_presets"
		printf "${C_BLUE}All available tags:${C_RESET}\n%s\n" "$all_tags"
		exit
		;;
	-lm | --list-modules)
		printf "${C_BLUE}All available modules:${C_RESET}\n%s\n" "$all_modules"
		exit
		;;
	-lp | --list-presets)
		printf "${C_BLUE}All available presets:${C_RESET}\n%s\n" "$all_presets"
		exit
		;;
	-lt | --list-tags)
		printf "${C_BLUE}All available tags:${C_RESET}\n%s\n" "$all_tags"
		exit
		;;
	-v | --verbose) # Verbose printing TODO: Pass to the dotmodules
		verbose=1
		;;
	-d | --dry) #  customize installable modules
		dry=1
		;;
	-f | --force) # force installation
		force=1
		;;
	-fp | --fix-permissions) # Adds executable rights to every scriptfile
		fix_permissions=1
		;;
	-c | --config | --custom) # Ask for everything
		config=1
		;;
	-?*)
		printf 'WARN: Unknown option (ignored): %s\n' "$1" >&2
		;;
	*) # Installs modules specified after it
		if [ -n "$1" ]; then
			modules_selected=$*
			[ $verbose = 1 ] && echo "Initially selected: $modules_selected"
		fi
		break
		;;
	esac

	shift
done

if [ $verbose = 1 ]; then
	echo "has_pacman? $has_pacman"
	echo "has_apt? $has_apt"
fi

trim_around() {
	# removes the first and last characters from every line
	rev | cut -c2- | rev | cut -c2-
}

has_tag() {
	# Returns every dotmodule that contains any of the tags
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

install_whatever() {
	while :; do
		if [ "$1" ]; then
			[ $verbose = 1 ] && echo "Trying to install $1..."
			# TODO: eval the dependency condition. Also setup supported vars
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
					if [ -z "$final_module_list" ]; then
						final_module_list="$1"
					else
						final_module_list="$final_module_list
$1"
					fi
					# install_module "$1"
					;;
				esac
				[ $verbose = 1 ] && echo "...done installing $1"
			else
				[ $verbose = 1 ] && echo "...already resolved $1"
			fi
			shift
		else
			break
		fi
	done
}

install_module() {
	while :; do
		if [ "$1" ]; then
			result=0
			[ $verbose = 1 ] && echo "Checking if module exists:" \
				"$modules_folder/$1"
			if [ ! -d "$modules_folder/$1" ]; then
				echo "Module $1 not found. Skipping"
				return 1
			fi

			# cd to dotmodule just in case a dotmodule
			# is not suited for installation outside of it
			# cd "${0%/*}" || return
			echo "${C_BLUE}Installing $1$C_RESET"

			# Only calculate the hashes if we going to use it
			if [ "$force" = 0 ]; then
				old_hash=$(cat "$modules_folder/$1/$hashfilename" 2>/dev/null)
				new_hash=$(tar --absolute-names \
					--exclude="$modules_folder/$1/$hashfilename" \
					-c "$modules_folder/$1" | sha1sum)
				[ $verbose = 1 ] && printf "$C_RED%s\n%s\n$C_RESET" \
					"$old_hash" \
					"$new_hash"
			fi

			if
				[ "$force" = 1 ] || [ "$old_hash" != "$new_hash" ]
			then
				[ $dry != 1 ] && echo "${C_CYAN}Applying dotmodule $1$C_RESET"

				if [ "$has_apt" ] &&
					[ -f "$modules_folder/$1/install.apt.sh" ]; then
					# shellcheck disable=SC2091
					if [ $dry != 1 ]; then
						(
							"$modules_folder/$1/install.apt.sh"
						)
						result=$((result + $?))
					fi

				fi
				if [ "$has_pacman" ] &&
					[ -f "$modules_folder/$1/install.pacman.sh" ]; then
					# shellcheck disable=SC2091
					if [ $dry != 1 ]; then
						[ $verbose = 1 ] && echo "Result before install.pacman.sh $1: $result"
						(
							"$modules_folder/$1/install.pacman.sh"
						)
						result=$((result + $?))
						[ $verbose = 1 ] && echo "Result after install.pacman.sh $1: $result"
					fi
				fi
				if [ -f "$modules_folder/$1/install.sudo.sh" ]; then
					# shellcheck disable=SC2091
					if [ $dry != 1 ]; then
						(
							"$modules_folder/$1/install.sudo.sh"
						)
						result=$((result + $?))
					fi
				fi
				if [ -f "$modules_folder/$1/install.sh" ]; then
					# shellcheck disable=SC2091
					if [ $dry != 1 ]; then
						[ $verbose = 1 ] && echo "Result before install.sh" \
							"$1: $result"
						(
							"$modules_folder/$1/install.sh"
						)
						result=$((result + $?))
						[ $verbose = 1 ] && echo "Result after install.sh" \
							"$1: $result"
					fi
				fi

				# Calculate fresh hash (always if not dryrunning)

				if [ $dry != 1 ]; then
					tar --absolute-names \
						--exclude="$modules_folder/$1/$hashfilename" \
						-c "$modules_folder/$1" |
						sha1sum >"$modules_folder/$1/$hashfilename"
					if [ $dry != 1 ] && [ "$result" = 0 ]; then
						printf "${C_GREEN}Successfully installed \
%s${C_RESET}\n" "$1"
					else
						printf "${C_RED}Installation failed \
%s${C_RESET}\n" "$1"
					fi
				fi

			else

				echo "$C_YELLOW! $1 is already installed and no changes" \
					"are detected$C_RESET"
			fi
			shift
		else
			break
		fi
	done
}

if [ -z "$modules_selected" ] || [ "$config" = 1 ]; then
	modules_selected=$(whiptail --title "Select modules to install" \
		--checklist "Space changes selection, enter approves" \
		0 0 0 zsh zsh ON vim vim OFF 3>&1 1>&2 2>&3 3>&- |
		sed 's/ /\n/g' |
		trim_around)
fi

# shellcheck disable=SC2086
install_whatever $modules_selected

[ $verbose = 1 ] && printf "${C_CYAN}Going to install:${C_RESET}\n%s\n" \
	"$final_module_list"

if [ "$fix_permissions" = 1 ]; then
	# Fix permissions
	echo "Fixing permissions..."
	# Alternative with shebang search: grep -rIzl '^#!' $modules_folder
	find $modules_folder -type f \
		-regex ".*\.\(sh\|zsh\|bash\|fish\|dash\)" -exec chmod +x {} \;
fi
# shellcheck disable=SC2086
install_module $final_module_list
