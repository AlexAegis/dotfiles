#!/bin/sh
# Loads the profile into the Plasma session, with a time limit

# Plasma waits for this file without a limit
plasma_env_limit=15
plasma_env_log="${XDG_STATE_HOME:-$HOME/.local/state}/plasma-env.log"

# Waits for the notification service
plasma_env_notify() {
	if ! command -v notify-send >/dev/null; then
		echo "plasma-env: notify-send not found" >>"$plasma_env_log"
		return 0
	fi
	# shellcheck disable=SC2016
	sh -c '
		i=0
		while [ "$i" -lt 120 ]; do
			if command -v busctl >/dev/null; then
				busctl --user status org.freedesktop.Notifications >/dev/null 2>&1 &&
					break
			elif [ "$i" -ge 30 ]; then
				break
			fi
			i=$((i + 1))
			sleep 1
		done
		notify-send -a "Plasma login" -u "$1" "$2" "$3"
	' sh "$@" </dev/null >/dev/null 2>&1 &
}

mkdir -p "${plasma_env_log%/*}" 2>/dev/null
# In a subshell: a failed redirect on : would end this shell
(: >"$plasma_env_log") 2>/dev/null ||
	plasma_env_log=$(mktemp 2>/dev/null) || plasma_env_log=/dev/null
plasma_env_file=$(mktemp -p "${XDG_RUNTIME_DIR:-/tmp}" plasma-env.XXXXXX 2>/dev/null) ||
	plasma_env_file=$(mktemp 2>/dev/null) || plasma_env_file=

plasma_env_status=0
if [ "$plasma_env_file" ]; then
	# shellcheck disable=SC2016
	timeout -k 5 "$plasma_env_limit" sh -c '
		QUIET=1
		. "$HOME/.profile"
		unset QUIET
		export PLASMA_ENV_LOADED=1
		export -p >&3
	' </dev/null >"$plasma_env_log" 2>&1 3>"$plasma_env_file" ||
		plasma_env_status=$?
fi

plasma_env_error=
if [ -z "$plasma_env_file" ]; then
	plasma_env_error="Could not create a temporary file."
elif [ "$plasma_env_status" -eq 124 ] || [ "$plasma_env_status" -eq 137 ]; then
	plasma_env_error="The profile timed out after $plasma_env_limit seconds."
elif [ "$plasma_env_status" -ne 0 ]; then
	plasma_env_error="The profile failed with status $plasma_env_status."
else
	# shellcheck disable=SC2016
	env -i /bin/sh -c '
		. "$1" && [ "$PLASMA_ENV_LOADED" = 1 ] || exit 2
		[ "${DBUS_SESSION_BUS_ADDRESS-}" = "$2" ] || exit 3
		[ "${XDG_RUNTIME_DIR-}" = "$3" ] || exit 3
		case ":$PATH:" in *:/usr/bin:*) ;; *) exit 3 ;; esac
		# Keep /usr/share and /usr/local/share if inherited
		for d in /usr/share /usr/local/share; do
			case ":$4:" in *:$d:* | *:$d/:*) ;; *) continue ;; esac
			case ":${XDG_DATA_DIRS:-/usr/local/share:/usr/share}:" in
			*:$d:* | *:$d/:*) ;;
			*) exit 3 ;;
			esac
		done
	' sh "$plasma_env_file" "${DBUS_SESSION_BUS_ADDRESS-}" "${XDG_RUNTIME_DIR-}" \
		"${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
	case $? in
	0) ;;
	2) plasma_env_error="The profile exited early." ;;
	*) plasma_env_error="The profile broke PATH, XDG_DATA_DIRS, D-Bus or XDG_RUNTIME_DIR." ;;
	esac
fi

if [ "$plasma_env_error" ]; then
	echo "plasma-env: $plasma_env_error" >>"$plasma_env_log"
	plasma_env_notify critical "Login environment not loaded" \
		"$plasma_env_error Session started without your profile. Log: $plasma_env_log"
else
	# shellcheck disable=SC1090
	. "$plasma_env_file"
	# Startup scripts that sourcein reported as failed
	plasma_env_failed=$(sed -n 's/^Return value of \(.*\) is [0-9][0-9]*$/\1/p' \
		"$plasma_env_log" 2>/dev/null | sed 's|.*/||' | tr '\n' ' ')
	if [ "$plasma_env_failed" ]; then
		plasma_env_notify normal "Startup scripts failed at login" \
			"Failed: ${plasma_env_failed% }. The rest of your profile loaded. Log: $plasma_env_log"
	fi
fi

[ -z "$plasma_env_file" ] || rm -f "$plasma_env_file"
unset plasma_env_limit plasma_env_log plasma_env_file plasma_env_status \
	plasma_env_error plasma_env_failed
unset -f plasma_env_notify
