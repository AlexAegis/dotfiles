#!/bin/sh
# Tests that each entry of the dependency, clash and preset files resolves

root=$(cd "$(dirname "$0")/.." && pwd) || exit 1
modules="$root/modules"
presets="$root/presets"
failures=0

tmp=$(mktemp -d) || exit 1
trap 'rm -rf -- "${tmp:?}"' EXIT
trap 'exit 1' HUP INT TERM

# shellcheck disable=SC2016
pont_line_awk='{
	line = $0
	sub(/#.*/, "", line)
	sub(/^[ \t\r]+/, "", line)
	sub(/[ \t\r]+$/, "", line)
}
line != "" { print FILENAME "\t" line }'
# shellcheck disable=SC2016
pont_entry_awk='{
	line = $0
	sub(/#.*/, "", line)
	sub(/^[ \t\r]+/, "", line)
	sub(/[ \t\r]+$/, "", line)
	sub(/[ \t\r]*[?].*/, "", line)
}
line != "" { print FILENAME "\t" line }'

pass() {
	printf 'ok: %s\n' "$1"
}

fail() {
	printf 'FAIL: %s\n' "$1"
	failures=$((failures + 1))
}

find "$modules" -mindepth 1 -maxdepth 1 -type d | sed 's|.*/||' \
	>"$tmp/modules" || exit 1
find "$presets" -type f -name '*.preset' | sed 's|.*/||; s|\.preset$||' \
	>"$tmp/presets" || exit 1
find "$modules" -mindepth 2 -maxdepth 2 -type f -name .tags \
	-exec awk "$pont_line_awk" {} + | cut -f 2 >"$tmp/tags" || exit 1
{
	find "$modules" -mindepth 2 -maxdepth 2 -type f \
		\( -name .dependencies -o -name .clash \) \
		-exec awk "$pont_entry_awk" {} + &&
		find "$presets" -type f -name '*.preset' \
			-exec awk "$pont_entry_awk" {} +
} >"$tmp/entries" || exit 1

checked=0
while IFS="$(printf '\t')" read -r file entry; do
	checked=$((checked + 1))
	case $entry in
		:*) list=tags name=${entry#:} kind=tag ;;
		+*) list=presets name=${entry#+} kind=preset ;;
		*) list=modules name=$entry kind=module ;;
	esac
	grep -qxF -- "$name" "$tmp/$list" ||
		fail "${file#"$root"/}: $entry names no $kind"
done <"$tmp/entries"

for list in modules presets tags entries; do
	[ -s "$tmp/$list" ] || fail "found no $list"
done
[ "$failures" -eq 0 ] &&
	pass "each of the $checked entries names a module, tag or preset"

[ "$failures" -eq 0 ] || {
	printf '%s test(s) failed\n' "$failures"
	exit 1
}
