#!/bin/sh

# Removes every word from $1 that occurs in $2
remove_words() {
	for a in $1; do
		local has=0
		for b in $2; do
			if [ "$a" = "$b" ]; then
				local has=1
			fi
		done
		if [ $has = 0 ]; then
			local res="$res $a"
		fi
	done

	echo "$res"
}

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
