#!/bin/sh

# Reset font info cache on linux systems
if command -v fc-cache; then
	fc-cache -vf
fi
