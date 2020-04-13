#!/bin/sh

# TODO: Finish, and style

result=$(echo "


" | rofi -dmenu)

# Add any other scripts that should happen on
# shutdown or attach them to systemd targets
case "$result" in
	)
		notify-send "Implement lock"
		;;
	)
		notify-send "Enable sleep"
		# systemctl suspend
		;;
	)
		choice=$(echo "No
Yes" | rofi -dmenu)
		if [ "$choice" = "Yes" ]; then
			notify-send "Enable logout"
			# bspc quit
		fi
		;;
	)
		choice=$(echo "No
Yes" | rofi -dmenu)
		if [ "$choice" = "Yes" ]; then
			notify-send "Enable reboot"
			# systemctl reboot
		fi
		;;
	)
		choice=$(echo "No
Yes" | rofi -dmenu)
		if [ "$choice" = "Yes" ]; then
			notify-send "Enable poweroff"
			# systemctl poweroff
		fi
		;;
esac
