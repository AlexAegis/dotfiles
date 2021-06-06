#!/bin/sh

# Copy vconsole fonts
if [ -d "$VCONSOLE_FONT_DIR" ]; then
	find "$POWERLINE_DIR" \
		-print0 \
		-regex '.*/PCF/.*\.pcfu?.gz' \
		-o -regex '.*/PSF/.*\.psfu?.gz' | while IFS=$(printf '\0') read -r file
	do
		cp "$file" "$VCONSOLE_FONT_DIR"
	done
fi

