#!/bin/bash
# Swaps stdout and stderr. Also closes the temporary 3rd channel.
3>&1 1>&2 2>&3 3>&-
# This not meant to be used as this file, but you can try it out:
#
# 	echo Hello | $(./scripts/swapstd.sh)
# 	>
#
