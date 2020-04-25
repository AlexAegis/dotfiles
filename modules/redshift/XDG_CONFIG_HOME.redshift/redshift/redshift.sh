#!/bin/sh

# Original from:
# https://github.com/VineshReddy/polybar-redshift/blob/master/redshift.sh

stateFile=${XDG_CACHE_HOME:-$HOME/.cache}/redshift/redshift_state
changeValue=300

if [ ! -e "$stateFile" ]; then
	mkdir -p "${XDG_CACHE_HOME:-$HOME/.cache}"/redshift/
	echo 'REDSHIFT=on
REDSHIFT_TEMP=6500
' > "$stateFile"
fi

# shellcheck disable=SC1090
. "$stateFile"

changeMode() {
	sed -i "s/REDSHIFT=$1/REDSHIFT=$2/g" "$stateFile"
	REDSHIFT=$2
	echo "$REDSHIFT"
}

changeTemp() {
	if [ "$2" -gt 1000 ] && [ "$2" -lt 25000 ]
	then
		sed -i "s/REDSHIFT_TEMP=$1/REDSHIFT_TEMP=$2/g" "$stateFile"
		redshift -x
		redshift -O $((REDSHIFT_TEMP+changeValue))
	fi
}

case $1 in
	toggle)
		if [ "$REDSHIFT" = on ];
		then
			changeMode "$REDSHIFT" off
			redshift -x
		else
			changeMode "$REDSHIFT" on
			redshift -O "$REDSHIFT_TEMP"
		fi
		;;
	increase)
		changeTemp $((REDSHIFT_TEMP)) $((REDSHIFT_TEMP+changeValue))
		;;
	decrease)
		changeTemp $((REDSHIFT_TEMP)) $((REDSHIFT_TEMP-changeValue));
		;;
	temperature)
		case $REDSHIFT in
		  on)
			printf "%dK" "$REDSHIFT_TEMP"
			;;
		  off)
			printf "off"
			;;
		esac
		;;
esac
