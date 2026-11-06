#!/bin/sh
# Tests of parse_args, its copy in timelock and the scripts that call it

root=$(cd "$(dirname "$0")/.." && pwd) || exit 1
bin="$root/modules/home/XDG_BIN_HOME.home"
failures=0

tmp=$(mktemp -d) || exit 1
watch_pid=
trap '[ -z "$watch_pid" ] || kill "$watch_pid" 2>/dev/null; rm -rf -- "${tmp:?}"' EXIT
trap 'exit 1' HUP INT TERM

mkdir "$tmp/glob" "$tmp/stub" "$tmp/watched" "$tmp/with space" \
	"$tmp/watch" "$tmp/watch/t[1]" "$tmp/watch/t1" || exit 1
: >"$tmp/glob/-x"
: >"$tmp/glob/ab"
: >"$tmp/watch/t[1]/f"
cat >"$tmp/stub/watch_exec" <<'EOF'
#!/bin/sh
printf '[%s]\n' "$@"
EOF
chmod +x "$tmp/stub/watch_exec"

pass() {
	printf 'ok: %s\n' "$1"
}

fail() {
	printf 'FAIL: %s\n' "$1"
	failures=$((failures + 1))
}

expect_output() {
	name="$1"
	expected="$2"
	shift 2
	actual=$("$@" 2>&1)
	if [ "$actual" = "$expected" ]; then
		pass "$name"
	else
		fail "$name"
		printf 'expected:\n%s\nactual:\n%s\n' "$expected" "$actual"
	fi
}

expect_status() {
	name="$1"
	expected="$2"
	shift 2
	"$@" >/dev/null 2>&1
	status=$?
	if [ "$status" = "$expected" ]; then
		pass "$name"
	else
		fail "$name"
		printf 'expected status %s, got %s\n' "$expected" "$status"
	fi
}

in_glob_dir() {
	(cd "$tmp/glob" && "$@")
}

timelock_parser=$(sed -n '/^expand_single_args() {/,/^}/p
/^parse_args() {/,/^}/p' "$root/modules/sys/XDG_BIN_HOME.sys/timelock")

timelock_parse_args() {
	(
		eval "$timelock_parser"
		[ "$(command -v parse_args)" = parse_args ] || exit 3
		parse_args "$@"
	)
}

script_parse_args() {
	sh "$bin/parse_args" "$@"
}

run_restart_on_change() {
	in_glob_dir env XDG_BIN_HOME="$tmp/stub" PATH="$bin:$PATH" sh \
		"$root/modules/systemd/XDG_CONFIG_HOME.systemd/systemd/restart_on_change" \
		"$@"
}

run_runic() {
	in_glob_dir env PATH="$bin:$PATH" sh "$root/modules/runic/runic.sh" "$@" \
		</dev/null
}

count_lines() {
	if [ -f "$1" ]; then
		wc -l <"$1" | tr -d ' '
	else
		echo 0
	fi
}

case $timelock_parser in
	*'expand_single_args() {'*'parse_args() {'*)
		pass 'timelock contains expand_single_args and parse_args'
		;;
	*)
		fail 'timelock contains expand_single_args and parse_args'
		;;
esac

for parser in script_parse_args timelock_parse_args; do
	expect_output "$parser keeps -n" '-n
-v' "$parser" '' -n -v
	expect_output "$parser splits a group with -n and -e" '-n
-v
-e' "$parser" '' -nve
	expect_output "$parser splits a value after =" '--log
3' "$parser" '' --log=3
	expect_output "$parser does not glob" '-*
a*' in_glob_dir "$parser" '' '-*' 'a*'
	expect_status "$parser refuses a flag without its value" 1 \
		"$parser" '-l' -l
done

expect_output 'restart_on_change passes the arguments of a watcher unit' "[-t]
[$tmp/watched]
[-e]
[/usr/bin/systemctl --user restart sxhkd]
[-d]
[0.5]
Restart on change terminated" \
	run_restart_on_change -t "$tmp/watched" -s sxhkd -d 0.5

expect_output 'restart_on_change applies -c and -r to an earlier -s' "[-t]
[$tmp/watched]
[-e]
[/usr/bin/systemctl  reload foo]
[-e]
[/usr/bin/systemctl  reload bar]
Restart on change terminated" \
	run_restart_on_change -s foo -c reload -t "$tmp/watched" -s bar -r

expect_output 'restart_on_change keeps spaces and globs in its values' "[-t]
[$tmp/with space]
[-e]
[/usr/bin/systemctl --user restart a*]
Restart on change terminated" \
	run_restart_on_change --target="$tmp/with space" --service='a*'

expect_status 'restart_on_change refuses -s without a value' 1 \
	run_restart_on_change -t "$tmp/watched" -s

expect_status 'watch_exec refuses -e without a value' 1 \
	env PATH="$bin:$PATH" timeout 5 sh "$bin/watch_exec" -t "$tmp/watched" -e

expect_status 'watch_exec refuses a target that is not a folder' 1 \
	env PATH="$bin:$PATH" timeout 5 sh "$bin/watch_exec" -t "$tmp/missing"

expect_line() {
	name="$1"
	expected="$2"
	actual=$(head -n 1 "$3" 2>/dev/null)
	if [ "$actual" = "$expected" ]; then
		pass "$name"
	else
		fail "$name"
		printf 'expected:\n%s\nactual:\n%s\n' "$expected" "$actual"
	fi
}

if [ -x /usr/bin/inotifywait ]; then
	failures_before_watch=$failures
	mkdir "$tmp/elsewhere" "$tmp/fake-sleep"
	: >"$tmp/spare"
	cat >"$tmp/fake-sleep/sleep" <<'EOF'
#!/bin/sh
printf '%s\n' "$1" >>"$WATCH_EXEC_TEST_DELAYS"
EOF
	chmod +x "$tmp/fake-sleep/sleep"
	(cd "$tmp/elsewhere" &&
		WATCH_EXEC_TEST_DELAYS="$tmp/delays" PATH="$tmp/fake-sleep:$bin:$PATH" \
			exec timeout 20 sh "$bin/watch_exec" -t "$tmp/watch/t[1]" \
			-e "echo hit >>'$tmp/hits'" \
			-e "printf '%s\n' '$tmp/glob/'a* >>'$tmp/globbed'" -d 7 \
			>"$tmp/watch_exec.log" 2>&1) &
	watch_pid=$!
	tries=0
	while [ "$(count_lines "$tmp/hits")" = 0 ] && [ "$tries" -lt 50 ]; do
		touch "$tmp/watch/t[1]/f"
		sleep 0.2
		tries=$((tries + 1))
	done
	sleep 1
	first_hits=$(count_lines "$tmp/hits")
	tries=0
	while [ "$(count_lines "$tmp/hits")" = "$first_hits" ] &&
		[ "$tries" -lt 25 ]; do
		if [ -e "$tmp/spare" ]; then
			mv "$tmp/spare" "$tmp/watch/t[1]/spare"
		else
			mv "$tmp/watch/t[1]/spare" "$tmp/spare"
		fi
		sleep 0.2
		tries=$((tries + 1))
	done
	last_hits=$(count_lines "$tmp/hits")
	kill "$watch_pid" 2>/dev/null
	watch_pid=

	if [ "$first_hits" -gt 0 ]; then
		pass 'watch_exec runs -e when a file in a folder named t[1] changes'
	else
		fail 'watch_exec runs -e when a file in a folder named t[1] changes'
	fi
	if [ "$last_hits" -gt "$first_hits" ]; then
		pass 'watch_exec still watches the folder named t[1] after a change'
	else
		fail 'watch_exec still watches the folder named t[1] after a change'
	fi
	expect_line 'watch_exec runs the -e commands with globbing on' \
		"$tmp/glob/ab" "$tmp/globbed"
	expect_line 'watch_exec sleeps for the -d value' 7 "$tmp/delays"
	[ "$failures" = "$failures_before_watch" ] || cat "$tmp/watch_exec.log"
else
	fail 'watch_exec: /usr/bin/inotifywait is missing (inotify-tools)'
fi

runic_stdout() {
	run_runic "$@" 2>/dev/null
}

runic_stderr() {
	{ run_runic "$@" >/dev/null; } 2>&1
}

expect_output 'runic keeps two spaces in an argument' '1  2' run_runic '1  2'
expect_output 'runic does not glob an argument' '*' run_runic '*'
expect_output 'runic keeps its output free of warnings' '7' runic_stdout -x 7
expect_output 'runic prints a warning for an unknown option on stderr' \
	'Unknown option (ignored): -x' runic_stderr -x 7

[ "$failures" = 0 ] || {
	printf '%s tests failed\n' "$failures"
	exit 1
}
