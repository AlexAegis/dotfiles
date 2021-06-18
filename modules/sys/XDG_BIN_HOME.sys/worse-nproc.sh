#!/bin/sh

# This is what you write if you don't know about `nproc`
lscpu | awk '$1 ~ /^CPU\(s\):/ { print $2 }'
