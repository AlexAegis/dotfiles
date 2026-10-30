#!/bin/sh

# KWin rule for the app ids that the proxy gives agent windows: the
# AgentWindow title bar, server-side decorations, and no focus stealing
rule=87047328-2cc4-4879-98c9-2b921a1e5ab0

set_key() {
	kwriteconfig6 --file kwinrulesrc --group "$1" --key "$2" "$3" || exit 1
}

# KWin applies the first rule that forces a property, so this rule goes first
put_first() {
	others=$(kreadconfig6 --file kwinrulesrc --group General --key "$1" |
		tr ',' '\n' | grep -vx "$rule" | paste -sd ,)
	set_key General "$1" "$rule${others:+,$others}"
}

set_key "$rule" Description 'Agent windows'
set_key "$rule" wmclass '^agent(\.|$)'
set_key "$rule" wmclassmatch 3
set_key "$rule" wmclasscomplete false
set_key "$rule" decocolor AgentWindow
set_key "$rule" decocolorrule 2
set_key "$rule" decorationpolicy server
set_key "$rule" decorationpolicyrule 2
set_key "$rule" fsplevel 4
set_key "$rule" fsplevelrule 2

# KWin 6.8 lists the rules in Order. It reads an older file by its rules key and
# drops the groups that the key does not list.
put_first Order
if [ -n "$(kreadconfig6 --file kwinrulesrc --group General --key rules)" ]; then
	put_first rules
	set_key General count \
		"$(kreadconfig6 --file kwinrulesrc --group General --key rules | tr ',' '\n' | grep -c .)"
fi

# Fails outside a running Plasma session, where KWin reads the rule at its start
dbus-send --session --type=method_call --dest=org.kde.KWin /KWin \
	org.kde.KWin.reconfigure 2>/dev/null || true
