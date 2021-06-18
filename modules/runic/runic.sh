#!/bin/sh

entries=
script='nordic'

show_version() {
	echo "Version: 0.1.0"
	# exit 0
}

show_help() {
	echo "runic $(show_version)
-h, --help                    -- Print information on usage and flags then
                                 exit
"
	exit 0
}

interpret_args() {
	IFS='
'
	while :; do
		case $1 in
			-h | -\? | --help) show_help ;;
			-v | --version) show_version ;;
			-s | --script)
				if [ "$2" = 'nordic' ] ||  [ "$2" = 'hungarian' ]; then
					script="$2"
				else
					echo "Invalid script: $2"; exit 1
				fi
				shift
				;;
			--)	;;
			-?*) echo "Unknown option (ignored): $1" b;;
			*)
				if [ "$1" ]; then
					if [ "$entries" ]; then
						entries="$entries${IFS:-\0}$1"
					else
						entries="$1"
					fi
				else
					break
				fi
				;;
		esac
		shift
	done
}

transform_entries() {
	while :; do
		[ "$1" ] || break
		[ "$char" ] && printf ' '

		tmp="$1"
		while [ -n "$tmp" ]; do
			rest="${tmp#?}"
			char="${tmp%"$rest"}"
			if [ "$script" = "nordic" ]; then
				case $char in
					a | A) printf 'ᚨ' ;;
					b | B) printf 'ᛒ' ;;
					c | C) printf 'ᚲ' ;;
					d | D) printf 'ᛞ' ;;
					e | E) printf 'ᛖ' ;;
					f | F) printf 'ᚠ' ;;
					g | G) printf 'ᚷ' ;;
					h | H) printf 'ᚺ' ;;
					i | I) printf 'ᛁ' ;;
					j | J) printf 'ᛃ' ;;
					k | K) printf 'ᚴ' ;;
					l | L) printf 'ᛚ' ;;
					m | M) printf 'ᛗ' ;;
					n | N) printf 'ᚾ' ;;
					o | O) printf 'ᛟ' ;;
					p | P) printf 'ᛈ' ;;
					q | Q) printf 'ᛩ' ;;
					r | R) printf 'ᚱ' ;;
					s | S) printf 'ᛋ' ;;
					t | T) printf 'ᛏ' ;;
					u | U) printf 'ᚢ' ;;
					v | V) printf 'ᚡ' ;;
					w | W) printf 'ᚹ' ;;
					x | X) printf 'ᛪ' ;;
					y | Y) printf 'ᚤ' ;;
					z | Z) printf 'ᛉ' ;;
					*) printf '%s' "$char" ;;
				esac
			fi
			tmp="$rest"
		done
		shift
	done
}

## Execution
# shellcheck disable=SC2046
interpret_args $(parse_args "" "$@")

# shellcheck disable=SC2086
transform_entries $entries

printf '
'
