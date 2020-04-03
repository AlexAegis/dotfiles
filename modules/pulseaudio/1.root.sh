#!/bin/sh

# TODO: Not working
file='/etc/pulse/daemon.conf'
line='deferred-volume-extra-delay-usec = 0'
grep -qFx -- "$line" "$file" || echo "$line" >>"$file"
