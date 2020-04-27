#!/bin/sh
echo I AM EXA 0 ROOT FALLBACK
env
env | grep CARGO_HOME
env | grep PATH
cargo install exa
