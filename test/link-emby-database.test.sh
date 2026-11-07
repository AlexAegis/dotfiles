#!/bin/sh
# Tests of link_emby_database.sh of the emby module

root=$(cd "$(dirname "$0")/.." && pwd) || exit 1
script="$root/modules/emby/XDG_BIN_HOME.emby/link_emby_database.sh"
failures=0

[ "$(id -u)" != 0 ] || {
	printf 'FAIL: run link-emby-database.test.sh as a user other than root\n'
	exit 1
}

tmp=$(mktemp -d) || exit 1
trap 'rm -rf -- "${tmp:?}"' EXIT
trap 'exit 1' HUP INT TERM

real_date=$(command -v date) || exit 1
real_mkdir=$(command -v mkdir) || exit 1
mkdir "$tmp/stub" "$tmp/failing-mv" "$tmp/failing-ln" "$tmp/racing-mkdir" ||
	exit 1
cat >"$tmp/racing-mkdir/mkdir" <<EOF
#!/bin/sh
"$real_mkdir" "\$LINK_EMBY_DATABASE_DATA_DIR" 2>/dev/null &&
	printf 'race' >"\$LINK_EMBY_DATABASE_DATA_DIR/library.db"
exec "$real_mkdir" "\$@"
EOF
printf '#!/bin/sh\nexit 1\n' >"$tmp/failing-ln/ln"
printf '#!/bin/sh\n' >"$tmp/stub/chown"
printf '#!/bin/sh\nexec %s -d "2001-02-03 04:05:06" "$@"\n' "$real_date" \
	>"$tmp/stub/date"
cat >"$tmp/failing-mv/mv" <<'EOF'
#!/bin/sh
mkdir -p "$2" && cp "$1/library.db" "$2/"
exit 1
EOF
chmod +x "$tmp/stub/chown" "$tmp/stub/date" "$tmp/failing-mv/mv" \
	"$tmp/failing-ln/ln" "$tmp/racing-mkdir/mkdir"
stubs="$tmp/stub"

pass() {
	printf 'ok: %s\n' "$1"
}

fail() {
	printf 'FAIL: %s\n' "$1"
	failures=$((failures + 1))
}

expect() {
	name="$1"
	shift
	if "$@"; then
		pass "$name"
	else
		fail "$name"
	fi
}

expect_refused() {
	name="$1"
	shift
	if run_link "$@"; then
		fail "$name"
	else
		pass "$name"
	fi
}

new_case() {
	case_dir="$tmp/$1"
	data="$case_dir/lib/emby"
	target="$case_dir/media/emby"
	mkdir -p "$case_dir/lib" || exit 1
}

run_link() {
	(
		cd "${case_dir:?}" &&
			LINK_EMBY_DATABASE_DATA_DIR="${data:?}" PATH="$stubs:$PATH" \
				sh "$script" "$@"
	) >/dev/null 2>"$tmp/stderr"
}

is_link_to() {
	[ -L "$data" ] && [ "$(readlink "$data")" = "$1" ] && [ -d "$1" ]
}

has_no_data_path() {
	[ ! -e "$data" ] && [ ! -L "$data" ]
}

count_backups() {
	find "$target" -mindepth 1 -maxdepth 1 -name 'prev-server-backup-*' |
		grep -c .
}

has_content() {
	[ -f "$1" ] && [ "$(cat "$1")" = "$2" ]
}

new_case move
mkdir "$data" && printf 'db' >"$data/library.db" || exit 1
expect 'runs with a target folder that does not exist' run_link "$target"
expect 'links the data folder to the moved folder' is_link_to "$target"
expect 'keeps the data after the move' has_content "$target/library.db" db
expect 'leaves no partial folder after the move' [ ! -e "$target.partial" ]
expect 'runs again with the same target folder' run_link "$target"
expect 'keeps the link on the second run' is_link_to "$target"
expect 'keeps the data on the second run' has_content "$target/library.db" db

new_case failed-move
mkdir "$data" && printf 'db' >"$data/library.db" || exit 1
stubs="$tmp/failing-mv:$tmp/stub"
expect_refused 'stops when the move fails' "$target"
stubs="$tmp/stub"
expect 'keeps the data folder when the move fails' \
	has_content "$data/library.db" db
expect 'keeps the data folder a folder when the move fails' [ ! -L "$data" ]
expect 'creates no target folder when the move fails' [ ! -e "$target" ]
expect_refused 'refuses to run again after a failed move' "$target"
expect 'keeps the data folder on the run after a failed move' \
	has_content "$data/library.db" db
expect 'keeps the partial folder for a check' [ -d "$target.partial" ]

new_case backup
mkdir -p "$data" "$target" || exit 1
printf 'old' >"$data/library.db" && printf 'new' >"$target/library.db" || exit 1
expect 'runs when both folders exist' run_link "$target"
expect 'links the data folder to the existing target folder' \
	is_link_to "$target"
expect 'keeps the data of the target folder' \
	has_content "$target/library.db" new
expect 'keeps the old data in a backup named by its date and time' \
	has_content "$target/prev-server-backup-2001-02-03-04:05:06/library.db" old
expect 'makes one backup in the target folder' \
	[ "$(count_backups)" = 1 ]

new_case relink
mkdir "$case_dir/old" && printf 'kept' >"$case_dir/old/library.db" || exit 1
ln -s "$case_dir/old" "$data" || exit 1
expect 'replaces a link to another folder' run_link "$target"
expect 'links the data folder to the new target folder' is_link_to "$target"
expect 'leaves the folder of the old link' \
	has_content "$case_dir/old/library.db" kept

new_case link-target-file
mkdir "$case_dir/old" "$case_dir/media" || exit 1
ln -s "$case_dir/old" "$data" && printf 'file' >"$target" || exit 1
expect_refused 'refuses a target file when the data folder is a link' "$target"
expect 'keeps the link when the target is a file' is_link_to "$case_dir/old"

new_case link-parent-file
mkdir "$case_dir/old" && ln -s "$case_dir/old" "$data" || exit 1
printf 'file' >"$case_dir/media" || exit 1
expect_refused 'refuses a target that cannot be created for a data link' \
	"$target"
expect 'keeps the link when the target cannot be created' \
	is_link_to "$case_dir/old"

new_case alias-target
mkdir "$case_dir/old" && printf 'kept' >"$case_dir/old/library.db" || exit 1
ln -s "$case_dir/old" "$data" && ln -s "$data" "$case_dir/alias" || exit 1
expect_refused 'refuses a target that leads through the data link' \
	"$case_dir/alias"
expect 'keeps the link after a target through it' is_link_to "$case_dir/old"
expect 'keeps the data after a target through the link' \
	has_content "$case_dir/old/library.db" kept

new_case racing-data-path
mkdir "$case_dir/old" && ln -s "$case_dir/old" "$data" || exit 1
stubs="$tmp/racing-mkdir:$tmp/stub"
expect_refused 'refuses when the data path comes back during the run' "$target"
stubs="$tmp/stub"
expect 'keeps a file that appears at the data path during the run' \
	has_content "$data/library.db" race

new_case alias-sub-target
mkdir "$case_dir/old" && ln -s "$case_dir/old" "$data" || exit 1
ln -s "$data" "$case_dir/alias" || exit 1
expect_refused 'refuses a folder in a target that leads through the data link' \
	"$case_dir/alias/sub"
expect 'keeps the link after a folder in a target through it' \
	is_link_to "$case_dir/old"
expect 'creates nothing in the old folder after a target through the link' \
	[ -z "$(ls -A "$case_dir/old")" ]

refuses_parent_alias_target() {
	new_case "parent-alias-$1"
	mkdir "$case_dir/old" && ln -s "$case_dir/old" "$data" || exit 1
	ln -s "$case_dir/lib" "$case_dir/lib-alias" || exit 1
	expect_refused "refuses the target lib-alias/$2 through a parent alias" \
		"$case_dir/lib-alias/$2"
	expect "keeps the link after the target lib-alias/$2" \
		is_link_to "$case_dir/old"
	expect "creates nothing in the old folder after the target lib-alias/$2" \
		[ -z "$(ls -A "$case_dir/old")" ]
}

refuses_parent_alias_target plain emby
refuses_parent_alias_target sub-folder emby/sub

new_case alias-target-failed-restore
mkdir "$case_dir/old" && ln -s "$case_dir/old" "$data" || exit 1
ln -s "$data" "$case_dir/alias" || exit 1
stubs="$tmp/failing-ln:$tmp/stub"
expect_refused 'refuses a target through the link when restoring fails' \
	"$case_dir/alias"
stubs="$tmp/stub"
expect 'names the old link target when the link cannot be restored' \
	grep -qF -- "-> $case_dir/old" "$tmp/stderr"

refuses_root_target() {
	new_case "root-$1"
	expect_refused "refuses the target $2" "$2"
	expect "creates no data link for the target $2" has_no_data_path
}

refuses_root_target plain /
refuses_root_target two-slashes //
refuses_root_target dot /.
refuses_root_target dot-dot /x/..

refuses_data_path_target() {
	new_case "data-path-$1"
	mkdir -p "$target" && ln -s "$target" "$data" || exit 1
	expect_refused "refuses the target $2 for the data folder lib/emby" \
		"$case_dir/$2"
	expect "keeps the link after the target $2" is_link_to "$target"
	expect "creates nothing in the linked folder after the target $2" \
		[ -z "$(ls -A "$target")" ]
}

refuses_data_path_target plain lib/emby
refuses_data_path_target slash lib/emby/
refuses_data_path_target two-slashes lib/emby//
refuses_data_path_target dot lib/emby/.
refuses_data_path_target inner-slashes lib//emby
refuses_data_path_target sub-folder lib/emby/sub

new_case fresh
expect 'runs without a data folder' run_link "$target"
expect 'creates and links the target folder' is_link_to "$target"

new_case no-argument
mkdir "$data" && printf 'db' >"$data/library.db" || exit 1
expect_refused 'refuses to run without a target folder'
expect 'keeps the data without a target folder' \
	has_content "$data/library.db" db

new_case relative
mkdir "$data" && printf 'db' >"$data/library.db" || exit 1
expect_refused 'refuses a relative target folder' media/emby
expect 'keeps the data with a relative target folder' \
	has_content "$data/library.db" db
expect 'creates nothing for a relative target folder' [ ! -e "$case_dir/media" ]

new_case data-file
printf 'file' >"$data" || exit 1
expect_refused 'refuses a data path that is a file' "$target"
expect 'keeps the file at the data path' has_content "$data" file
expect 'creates no target folder for a data file' [ ! -e "$target" ]

new_case target-file
mkdir -p "$data" "${target%/*}" && printf 'db' >"$data/library.db" || exit 1
printf 'file' >"$target" || exit 1
expect_refused 'refuses a target that is a file' "$target"
expect 'keeps the data with a target file' has_content "$data/library.db" db
expect 'keeps the target file' has_content "$target" file

[ "$failures" -eq 0 ] || {
	printf '%s test(s) failed\n' "$failures"
	exit 1
}
