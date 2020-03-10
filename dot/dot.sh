#!/bin/sh

#      _       _
#   __| | ___ | |_
#  / _` |/ _ \| __|
# | (_| | (_) | |_
#  \__,_|\___/ \__|
#
# A simple dotmodule manager
# TODO: Rice this script https://stackoverflow.com/questions/430078
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

# TODO: Enable line end comments (just strip #.*$ from every line)
# TODO: Uninstall by default unstow, if full uninstall then run the uninstall
# TODO: scripts, and uninstall should also remove the .tarhash file

# TODO: Clash support. Use .clash file, if two modules clash, ask which to use
# TODO: If a clashing module is already installed, abort, ask if interactive,
# TODO: remove other if forced

# TODO: clash feature support tags, see if something from that tag is installed

# TODO: Auto unstow at end of uninstall

# TODO: track dangling dependencies. When installing leave a file in the module
# TODO: that will store a snapshot of the dependencies. During uninstall check
# TODO: If there is a dependency somewhere that is not directly installed.
# TODO: (Or maybe dont and leave this to dot2)

# TODO: Before installing a module (Right before autostowing)
# TODO: Create every folder in the module as an empty folder in $TARGET

# TODO: If the module containes a git submodule. Check it out / update it

# TODO: Experiment with paralell execution (sort dependencies into a tree)
# TODO: Right now every dependency is sorted into a column and executed
# TODO: sequentially. The new executor would treat everything in one column
# TODO: and one indentation as modules that can be executed paralelly
# TODO: then pass everything below as a dependency list to it with one level
# TODO: of indentation removed
# TODO: make a test modules directory with modules with logs and sleeps
# TODO: Also a buffer output is needed to display this
# TODO: It should keep a buffer as high as many modules are currently being
# TODO: installed and then do the logging above it like normal
# TODO: Or have an indented section below each entry with a preview log
# TODO: or both
# TODO: investigate if feasible
# TODO: add flags to disable or enable paralell work
# TODO: add a .lock file with PID into each module just in case and remove after

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

# Environment
# script_path="$(dirname "$(readlink -f "$0")")"
dotfiles_folder="${DOTFILES-"$HOME/.dotfiles"}"
user_home=$(getent passwd "${SUDO_USER-$USER}" | cut -d: -f6)

# Config
IFS='
'
modules_selected=""
resolved=""
final_module_list=""
config=0
skip_root=0
force=0
update=0
remove=0
all=0
no_expand=0
fix_permissions=0
# TODO: Support multiple folders
modules_folder=${DOT_MODULES_FOLDER:-"$dotfiles_folder/modules"}
presets_folder=${DOT_PRESETS_FOLDER:-"$dotfiles_folder/presets"}
preset_extension=".preset"
hashfilename=".tarhash"
dependenciesfilename=".dependencies"
tagsfilename=".tags"
verbose=0 # Print more
dry=0     # When set, no installation will be done

# Config file sourcing
if [ -e "$user_home/.config/dot/dotrc" ]; then
	# shellcheck disable=SC1090
	. "$user_home/.config/dot/dotrc"
fi

if [ -e "$user_home/.config/dotrc" ]; then
	# shellcheck disable=SC1090
	. "$user_home/.config/dotrc"
fi

if [ -e "$user_home/.dotrc" ]; then
	# shellcheck disable=SC1090
	. "$user_home/.dotrc"
fi

if [ -e "./.dotrc" ]; then
	# shellcheck disable=SC1090,SC1091
	. "./.dotrc"
fi

# TODO: Only needed when printing and using whiptail. Lazy load it.
all_modules=$(find "$modules_folder/" -maxdepth 1 -mindepth 1 -printf "%f\n" |
	sort)
all_presets=$(find "$presets_folder/" -maxdepth 1 -mindepth 1 \
	-name '*.preset' -printf "%f\n" | sed 's/.preset//' | sort)
#shellcheck disable=SC2016
all_installed=$(grep -lm 1 -- "" "$modules_folder"/**/$hashfilename |
	sed -r 's_^.*/([^/]*)/[^/]*$_\1_g' | sort)
all_tags=$(find "$modules_folder"/*/ -maxdepth 1 -mindepth 1 -name '.tags' \
	-exec cat {} + | grep "^[^#;]" | sort | uniq)

# Package manager availablity
pacman=$(is_installed pacman)
apt=$(is_installed apt)
sysctl=$(is_installed sysctl)
systemctl=$(is_installed systemctl)
systemd=$systemctl
# TODO: Only valid on systemd distros
distribution=$(grep "^NAME" /etc/os-release | grep -oh "=.*" | tr -d '="')
arch=$(if [ "$distribution" = 'Arch Linux' ]; then echo 1; fi)
void=$(if [ "$distribution" = 'Void Linux' ]; then echo 1; fi)
debian=$(if [ "$distribution" = 'Debian GNU/Linux' ]; then echo 1; fi)
ubuntu=$(if [ "$distribution" = 'Ubuntu' ]; then echo 1; fi)
fedora=$(if [ "$distribution" = 'Fedora' ]; then echo 1; fi)

## Argument handling

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
		printf "${C_BLUE}All available environmental variables\
:${C_RESET}\n%s\n" "$all_tags"
		exit
		;;
	-li | --list-installed)
		printf "${C_BLUE}All installed modules:${C_RESET}\n%s\n" \
			"$all_installed"
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
	-le | --list-environment)
		echo "${C_BLUE}All available environmental variables:${C_RESET}
\$distribution: $distribution (Value of NAME in /etc/os-release)
\$pacman: $pacman
\$apt: $apt
\$sysctl: $sysctl
\$systemctl: $systemctl
\$systemd: $systemd
\$arch: $arch (is set when \$distribution is 'Arch Linux')
\$void: $void (is set when \$distribution is 'Void Linux')
\$debian: $debian (is set when \$distribution is 'Debian GNU/Linux')
\$ubuntu: $ubuntu (is set when \$distribution is 'Ubuntu')
\$fedora: $fedora (is set when \$distribution is 'Fedora')

anything you find in dot.sh
anything you pass to it. For example:
	TEST=1 dot 'base ? [ \$TEST = 1 ]' # this would pass
	TEST=1 dot 'base ? [ \$TEST = 0 ]' # this do not
"
		exit
		;;
	-v | --verbose) # Verbose printing TODO: Pass to the dotmodules
		verbose=1
		;;
	-u | --update) # Run only update scripts
		update=1
		;;
	-r | --remove) # Run only remove scripts
		remove=1
		# TODO: until orphan handling is done, do not touch other modules
		no_expand=1
		;;
	-a | --all) # Run all modules
		all=1
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
	-ne | --no-expand) # Disables dependency resolvement, run only selected
		no_expand=1
		;;
	-sr | --skip-root) # Skip root scripts
		skip_root=1
		;;
	-sc | -cpt | --scaffold) # Ask for everything
		# TODO: cpt template and dot --scaffold command to create from template
		# TODO: Use the remaining inputs as module folders to scaffold using cpt
		# TODO: If cpt is not available then try install it with cargo first
		# TODO: If no cargo is available then prompt the user to install it
		exit 0
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

do_fix_permissions() {
	# Fix permissions, except in submodules
	echo "Fixing permissions in $dotfiles_folder... "
	submodules=$(
		cd "$dotfiles_folder" || exit
		git submodule status | sed -e 's/^ *//' -e 's/ *$//' | rev |
			cut -d ' ' -f 2- | rev | cut -d ' ' -f 2- |
			sed -e 's@^@-not -path "**/@' -e 's@$@/*"@' | tr '\n' ' '
	)

	eval "find $modules_folder -type f \( $submodules \) \
-regex '.*\.\(sh\|zsh\|bash\|fish\|dash\)' -exec chmod +x {} \;"
}

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

get_entry() {
	echo "$1" | cut -d '?' -f 1 | sed 's/ $//'
}

get_condition() {
	echo "$1" | cut -d '?' -s -f 2- | sed 's/^ //'
}

execute_scripts_for_module() {
	for script in $2; do
		echo "Running $script..."

		privilige=$(echo "$script" | cut -d '.' -f 2 |
			sed 's/-.*//')

		if [ $dry != 1 ]; then
			if [ "$privilige" = "root" ] ||
				[ "$privilige" = "sudo" ]; then
				if [ "$skip_root" = 0 ]; then
					(
						sudo "$modules_folder/$1/$script"
					)
				else
					echo "${C_YELLOW}Skipping $script${C_RESET}"
				fi
			else
				if [ "$SUDO_USER" ]; then
					(
						sudo -u "$SUDO_USER" "$modules_folder/$1/$script"
					)
				else
					(
						"$modules_folder/$1/$script"
					)
				fi
			fi
			result=$((result + $?))
		fi
	done
}

expand_entry() {
	while :; do
		if [ "$1" ]; then
			# Extracting condition, if there is
			condition="$(get_condition "$1")"

			if [ $verbose = 1 ]; then
				echo "${C_YELLOW}Trying to install $(get_entry "$1")...$C_RESET"
				[ "$condition" ] &&
					echo "${C_YELLOW}...with condition $condition...$C_RESET"
			fi

			if ! eval "$condition"; then
				echo "${C_YELLOW}Condition ($condition) for $1" \
					"did not met, skipping$C_RESET"
				shift
				continue
			fi

			if [ "$(echo "$resolved" | grep "$1")" = "" ]; then
				resolved="$resolved
$1"
				case "$1" in
				+*) # presets
					# shellcheck disable=SC2046
					expand_entry $(in_preset "$(get_entry "$1" | cut -c2-)")
					;;
				:*) # tags
					# shellcheck disable=SC2046
					expand_entry $(has_tag "$(get_entry "$1" | cut -c2-)")
					;;
				*) # modules
					# shellcheck disable=SC2046
					expand_entry $(get_dependencies "$(get_entry "$1")")
					if [ -z "$final_module_list" ]; then
						final_module_list="$(get_entry "$1")"
					else
						final_module_list="$final_module_list
$(get_entry "$1")"
					fi
					;;
				esac
				[ $verbose = 1 ] && echo "...done resolving $1"
			else
				[ $verbose = 1 ] && echo "...already resolved $1"
			fi
			shift
		else
			break
		fi
	done
}

init_module() {
	init_sripts_in_module=$(find "$modules_folder/$1/" -type f \
		-regex "^.*/init\..*\.sh$" | sed 's|.*/||' | sort)
	execute_scripts_for_module "$1" "$init_sripts_in_module"
}

update_modules() {
	while :; do
		if [ "$1" ]; then
			update_sripts_in_module=$(find "$modules_folder/$1/" -type f \
				-regex "^.*/update\..*\.sh$" | sed 's|.*/||' | sort)
			execute_scripts_for_module "$1" "$update_sripts_in_module"
			shift
		else
			break
		fi
	done
}

remove_modules() {
	while :; do
		if [ "$1" ]; then
			remove_sripts_in_module=$(find "$modules_folder/$1/" -type f \
				-regex "^.*/remove\..*\.sh$" | sed 's|.*/||' | sort)
			execute_scripts_for_module "$1" "$remove_sripts_in_module"

			# unstow
			if [ "$SUDO_USER" ]; then
				sudo -E -u "$SUDO_USER" \
					stow -D -d "$modules_folder/$1/" \
					-t "$user_home" ".$1"
			else
				stow -D -d "$modules_folder/$1/" \
					-t "$user_home" ".$1"
			fi

			# remove hashfile to mark as uninstalled
			[ -e "$modules_folder/$1/$hashfilename" ] &&
				rm "$modules_folder/$1/$hashfilename"

			shift
		else
			break
		fi
	done
}

stow_module() {
	if [ $dry != 1 ] && [ -e "$modules_folder/$1/.$1" ]; then
		[ $verbose = 1 ] &&
			echo "Stowing .$1 in $modules_folder/$1/ to $user_home"

		if [ "$SUDO_USER" ]; then
			sudo -E -u "$SUDO_USER" \
				stow -d "$modules_folder/$1/" \
				-t "$user_home" ".$1"
		else
			stow -d "$modules_folder/$1/" \
				-t "$user_home" ".$1"
		fi

	fi
}

install_module() {
	sripts_in_module=$(find "$modules_folder/$1/" -type f \
		-regex "^.*/[0-9\]\..*\.sh$" | sed 's|.*/||' | sort)

	[ $verbose = 1 ] && echo "Scripts in module for $1 are:
$sripts_in_module"
	sripts_to_almost_run=
	for script in $sripts_in_module; do
		direct_dependency=$(echo "$script" | cut -d '.' -f 3)
		if [ "$(command -v "$direct_dependency" 2>/dev/null)" ] ||
			[ "$direct_dependency" = "fallback" ]; then
			sripts_to_almost_run="$sripts_to_almost_run
$script"
		fi
	done
	sripts_to_run=
	for script in $sripts_to_almost_run; do
		index=$(echo "$script" | cut -d '.' -f 1 |
			sed 's/-.*//')
		direct_dependency=$(echo "$script" | cut -d '.' -f 3)
		# Only keep fallbacks if they are alone in their index
		if [ "$direct_dependency" = "fallback" ]; then
			if [ "$(echo "$sripts_to_almost_run" |
				grep -c "$index.*")" = 1 ]; then
				sripts_to_run="$sripts_to_run
$script"
			fi
		else
			sripts_to_run="$sripts_to_run
$script"
		fi
	done
	[ $verbose = 1 ] && echo "Scripts to run for $1 are:
$sripts_to_run"

	# Run the resulting script list
	execute_scripts_for_module "$1" "$sripts_to_run"
}

hash_module() {
	if [ "$SUDO_USER" ]; then
		sudo -E -u "$SUDO_USER" \
			tar --absolute-names \
			--exclude="$modules_folder/$1/$hashfilename" \
			-c "$modules_folder/$1" |
			sha1sum >"$modules_folder/$1/$hashfilename"
	else
		tar --absolute-names \
			--exclude="$modules_folder/$1/$hashfilename" \
			-c "$modules_folder/$1" |
			sha1sum >"$modules_folder/$1/$hashfilename"
	fi

	if [ $dry != 1 ] && [ "$result" = 0 ]; then
		printf "${C_GREEN}Successfully installed \
%s${C_RESET}\n" "$1"
	else
		printf "${C_RED}Installation failed \
%s${C_RESET}\n" "$1"
	fi
}

execute_modules() {
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
			cd "$modules_folder/$1" || return 1

			echo "${C_BLUE}Installing $1$C_RESET"

			# Only calculate the hashes if we going to use it
			if [ "$force" = 0 ]; then
				old_hash=$(cat "$modules_folder/$1/$hashfilename" 2>/dev/null)
				new_hash=$(tar --absolute-names \
					--exclude="$modules_folder/$1/$hashfilename" \
					-c "$modules_folder/$1" | sha1sum)

				if [ "$old_hash" = "$new_hash" ]; then
					match=$C_GREEN
				fi

				[ $verbose = 1 ] && printf "${match-$C_RED}%s\n%s\n$C_RESET" \
					"$old_hash" \
					"$new_hash"
			fi

			if
				[ "$force" = 1 ] || [ "$old_hash" != "$new_hash" ]
			then

				if [ -e "$modules_folder/$1/.deprecated" ]; then
					echo "${C_YELLOW}! Warning: $1 is deprecated$C_RESET"
					shift
					continue
				fi

				[ "$dry" = 1 ] && echo "${C_RED}$1 would be installed!$C_RESET"

				[ "$dry" != 1 ] && echo "${C_CYAN}Applying dotmodule $1$C_RESET"

				init_module "$1"

				stow_module "$1"

				install_module "$1"

				# Calculate fresh hash (always if not dryrunning)

				if [ $dry != 1 ]; then
					hash_module "$1"
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

if [ "$all" = 0 ] && [ -z "$modules_selected" ] || [ "$config" = 1 ]; then
	modules_selected=$(whiptail --title "Select modules to install" \
		--checklist "Space changes selection, enter approves" \
		0 0 0 zsh zsh ON vim vim OFF 3>&1 1>&2 2>&3 3>&- |
		sed 's/ /\n/g' |
		trim_around)
fi

## Execution

# shellcheck disable=SC2086
if [ "$all" = 1 ]; then
	expand_entry $all_modules
else
	# shellcheck disable=SC2086
	if [ "$no_expand" = 1 ]; then
		final_module_list=$modules_selected
	else
		expand_entry "base" $modules_selected
	fi
fi

[ $verbose = 1 ] && printf "${C_CYAN}Going to install:${C_RESET}\n%s\n" \
	"$final_module_list"

if [ "$fix_permissions" = 1 ]; then
	do_fix_permissions
fi

if [ "$remove" = 1 ]; then
	# shellcheck disable=SC2086
	remove_modules $final_module_list
elif [ "$update" = 1 ]; then
	# shellcheck disable=SC2086
	update_modules $final_module_list
else
	# shellcheck disable=SC2086
	execute_modules $final_module_list
fi
