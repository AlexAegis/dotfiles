#!/bin/sh

available='us
hu'

pat='layout: *'
current_layout=$(setxkbmap -print -verbose 7 | sed -n "/$pat/s/$pat//p")


next=$(echo "$available" | sed -n "/$current_layout/{n;p;}")
next=${next:-$(echo "$available" | sed 1q)}

notify-send "Layout changed to: $next"
setxkbmap "$next"

