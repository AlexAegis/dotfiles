#!/bin/sh

# skip if already installed (it errors out if it is)
if ! command -v xcodebuild; then
	xcode-select --install
fi
