#!/bin/sh

testdata='
[sectionn]
a=4
c=3
# [sectiona]
[sectiona]
a=42
# b=543
[sectionb]
a=1
b=1
'
# first script, do it with uncommented sections
# if in an uncommented section a commented or uncommented key is found
#   uncomment it and set the value
#  if an uncommented line with the same key in the same section is found
#   comment it out so its unique
# then if no uncommented section is found, run second script, search for
# commented sections. If an uncommented key is found in that section
# comment it out, set the value. The section was commented out so no other
# keys should be there uncommented. But if the file was scrambled, that could
# be the part of the previous one
editini() {
	# $1 section
	# $2 key
	# $3 value
	echo "$testdata" | awk "
		BEGIN {
			SECTION = \"\"
		}
		{
			printf(\"section:\", SECTION)
			print \"\"
		}
		/^ *\[$1\] *$/ {
			# if found mark as found
			# if subsequent is found and not commented, comment it
			printf(\"SECTION SET:\")
			SECTION=\$0
			print SECTION
		}
		/^ *$2 *=.*$/ {
			printf(\"FOUND\")
			print \"SECTION\"
			print $1
			print \$0
			if (length(SECTION) == 0) {
				printf(\"OOOOOOOOOOOOOOOO\")
				print \$0
			}
		}
		/^ *\[.*\] *$/ {
			# if found mark as found
			# if subsequent is found and not commented, comment it
			printf(\"UNSET?\")
			print \$0
			if (length(SECTION) != 0) {
				printf(\"UNSET\")
				SECTION=\"\"

			}
		}

	"

}

editini "sectiona" "a" "10"
