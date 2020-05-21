#!/bin/sh

# Pipable trimmer
# By default trims spaces
trim() {
	rev | cut -d "${1- }" -f 2- | rev | cut -d "${1- }" -f 2-
}

# returns only the parent folders of the matches
# Example `matching_parents ~/.dotfiles/modules/**/.tarhash`
# would return every dotmodule that has a .tarhash file
matching_parents() {
	IFS='
'
	# shellcheck disable=SC2016
	grep -lm 1 -- "" "$@" | sed -r 's_^.*/([^/]*)/[^/]*$_\1_g'
}

to_absolute() {
	# Coerces the argument as an absolute path
	if [ "$1" ]; then
		if [ "${1##/}" = "$1" ]; then
			echo "$PWD/$1"
		else
			echo "$1"
		fi
	else
		echo "$PWD"
	fi
}

is_image() {
	# pipable function, checks each file from stdin at pwd or the
	# argument that its an image or not
	while read -r input; do
		abs_path="$(to_absolute "$1")/$input"
      	if file -b "$abs_path" | grep image > /dev/null; then
			echo "$abs_path"
		fi
  	done
}

last_modified_image() {
	# returns the last modified images absolute path at the argument
	target_dir="$(to_absolute "$1")"
	/bin/ls -t1 "$target_dir" | is_image "$target_dir" | head -n 1
}
