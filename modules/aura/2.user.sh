#!/bin/sh

mkdir -p "$XDG_CACHE_HOME"/aura/builds
mkdir -p "$XDG_CACHE_HOME"/aura/cache
mkdir -p "$XDG_CACHE_HOME"/aura/hashes
mkdir -p "$XDG_CACHE_HOME"/aura/packages

aura check
