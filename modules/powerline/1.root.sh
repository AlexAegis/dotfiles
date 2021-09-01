#!/bin/sh

# Copy vconsole fonts
if [ -d "$VCONSOLE_FONT_DIR" ]; then
	find "$POWERLINE_DIR" \
		-regex '.*/PCF/.*\.pcfu?.gz' \
		-o -regex '.*/PSF/.*\.psfu?.gz' | while read -r file
	do
		cp "$file" "$VCONSOLE_FONT_DIR"
	done
fi

