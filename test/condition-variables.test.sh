#!/bin/sh
# Tests that each $name in a condition has a setter: pont, a pontrc, a user i
# or e script, the file itself or the session. Entry guards run before any
# script, so scripts do not count there. Not checked: the order of setters and
# uses, a run under sudo pont (user scripts then set nothing), a bare name in
# $(( )).

root=$(cd "$(dirname "$0")/.." && pwd) || exit 1
modules="$root/modules"
presets="$root/presets"
pont="$modules/pont/pont.sh"
failures=0

tmp=$(mktemp -d) || exit 1
trap 'rm -rf -- "${tmp:?}"' EXIT
trap 'exit 1' HUP INT TERM

# shellcheck disable=SC2016
assignment_awk='/^[ \t]*([^=()#]*\)[ \t]*)?(export[ \t]+)?[A-Za-z_][A-Za-z0-9_]*=/ {
	name = $0
	sub(/^[ \t]*([^=()#]*\)[ \t]*)?(export[ \t]+)?/, "", name)
	sub(/=.*/, "", name)
	print name
}'
# Prints kind, file and name of each use that the file does not set itself
# shellcheck disable=SC2016
used_names_awk='{
	line = " " $0
	entry = FILENAME ~ /[.](dependencies|clash|preset)$/
	if (FILENAME ~ /[^\/][.]condition$/)
		sub(/[ \t;&|()]#.*/, "", line)
	else
		sub(/#.*/, "", line)
	if (entry && !sub(/^[^?]*[?]/, " ", line))
		next
	rest = line
	while (match(rest, /[^A-Za-z0-9_$][A-Za-z_][A-Za-z0-9_]*=/)) {
		set[FILENAME "\t" substr(rest, RSTART + 1, RLENGTH - 2)] = 1
		rest = substr(rest, RSTART + RLENGTH)
	}
	rest = line
	while (match(rest, /[^A-Za-z0-9_]for[ \t]+[A-Za-z_][A-Za-z0-9_]*/)) {
		name = substr(rest, RSTART + 1, RLENGTH - 1)
		sub(/^for[ \t]+/, "", name)
		set[FILENAME "\t" name] = 1
		rest = substr(rest, RSTART + RLENGTH)
	}
	rest = line
	while (match(rest, /[^A-Za-z0-9_]read([ \t]+[-A-Za-z0-9_]+)+/)) {
		count = split(substr(rest, RSTART + 1, RLENGTH - 1), words, /[ \t]+/)
		for (i = 2; i <= count; i++)
			if (words[i] !~ /^-/)
				set[FILENAME "\t" words[i]] = 1
		rest = substr(rest, RSTART + RLENGTH)
	}
	rest = line
	while (match(rest, /[$]([{]#?)?[A-Za-z_][A-Za-z0-9_]*/)) {
		name = substr(rest, RSTART + 1, RLENGTH - 1)
		sub(/^[{]#?/, "", name)
		used[(entry ? "entry" : "module") "\t" FILENAME "\t" name] = 1
		rest = substr(rest, RSTART + RLENGTH)
	}
}
END {
	for (key in used) {
		split(key, part, "\t")
		if (!((part[2] "\t" part[3]) in set))
			print key
	}
}'

pass() {
	printf 'ok: %s\n' "$1"
}

fail() {
	printf 'FAIL: %s\n' "$1"
	failures=$((failures + 1))
}

[ -f "$pont" ] || {
	fail "modules/pont/pont.sh missing: git submodule update --init modules/pont"
	exit 1
}

awk '/^set -a/ { inside = 1; next } /^set \+a/ { exit } inside' "$pont" \
	>"$tmp/pont-exports" || exit 1
awk "$assignment_awk" "$tmp/pont-exports" >"$tmp/pont-set" || exit 1
find "$modules" -mindepth 2 -maxdepth 2 -type f >"$tmp/module-files" || exit 1
find "$modules" -mindepth 2 -maxdepth 2 -name '*$*' >"$tmp/segment-paths" ||
	exit 1
# The user i and e scripts, by the name rule of pont. pont sources only these.
awk '{ name = $0; sub(/.*\//, "", name) }
name ~ /^(e|env|i|init)[0-9]*[.]user(-[^.]*)?[.]([^.]*[.])*sh$/' \
	"$tmp/module-files" >"$tmp/scripts" || exit 1
find "$modules" -type f \( -name pontrc -o -name .pontrc \) >"$tmp/pontrcs" ||
	exit 1
while IFS= read -r file; do
	awk "$assignment_awk" "$file" || exit 1
done <"$tmp/pontrcs" >"$tmp/pontrc-set" || exit 1
while IFS= read -r file; do
	awk "$assignment_awk" "$file" || exit 1
done <"$tmp/scripts" >"$tmp/script-set" || exit 1
# Names that the session gives pont, such as HOME and SUDO_USER
printf '%s\n' HOME LOGNAME PATH PWD SHELL TERM USER SUDO_USER SUDO_UID \
	SUDO_GID >"$tmp/session-set" || exit 1
sort -u "$tmp/pont-set" "$tmp/pontrc-set" "$tmp/session-set" \
	>"$tmp/entry-set" || exit 1
sort -u "$tmp/entry-set" "$tmp/script-set" >"$tmp/module-set" || exit 1

{
	find "$modules" -mindepth 2 -maxdepth 2 -type f \
		\( -name '*.condition' -o -name .dependencies -o -name .clash \) \
		-exec awk "$used_names_awk" {} + &&
		find "$presets" -type f -name '*.preset' \
			-exec awk "$used_names_awk" {} + &&
		awk '{
			rest = $0
			sub(/.*\//, "", rest)
			while (match(rest, /[$][A-Za-z_][A-Za-z0-9_]*/)) {
				print "module\t" $0 "\t" substr(rest, RSTART + 1, RLENGTH - 1)
				rest = substr(rest, RSTART + RLENGTH)
			}
		}' "$tmp/segment-paths"
} >"$tmp/used.all" || exit 1
sort -u "$tmp/used.all" >"$tmp/used" || exit 1

checked=0
while IFS="$(printf '\t')" read -r kind file name; do
	checked=$((checked + 1))
	grep -qxF -- "$name" "$tmp/$kind-set" && continue
	case $kind in
		entry) setters='pont, a pontrc and the session' ;;
		*) setters='pont, a pontrc, a user i or e script, the file and the session' ;;
	esac
	fail "${file#"$root"/}: \$$name, set by none of $setters"
done <"$tmp/used"

[ -s "$tmp/pont-set" ] ||
	fail "found no assignments in the set -a block of pont.sh"
[ -s "$tmp/used" ] || fail "found no names used in conditions"
[ "$failures" -eq 0 ] &&
	pass "each of the $checked names, counted per file, has a setter"

[ "$failures" -eq 0 ] || {
	printf '%s test(s) failed\n' "$failures"
	exit 1
}
