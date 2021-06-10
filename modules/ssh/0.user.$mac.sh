#!/bin/sh


# ssh itself is preinstalled but you might need `connect`, which is not
# netcat is available though, but it fails in some cases
brew install connect

# The condition on this file is $mac because brew doesn't imply mac, but
# mac does imply brew because it's a dependency of sys-mac
