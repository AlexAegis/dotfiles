#compdef _hello hello

function _hello() {
	local line

	_arguments -C \
		"-h[Show help information]" \
		"--h[Show help information]" \
		"1: :(quietly loudly)" \
		"*::arg:->args"

	case $line[1] in
	loudly)
		_hello_loudly
		;;
	quietly)
		_hello_quietly
		;;
	esac
}

function _hello_quietly() {
	_arguments \
		"--silent[Dont output anything]"
}

function _hello_loudly() {
	_arguments \
		"--repeat=[Repat the <message> any number of times]"
}
