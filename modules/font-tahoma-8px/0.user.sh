#!/bin/sh

FONTS_DIR="${XDG_DATA_HOME}/fonts"
mkdir -p "${FONTS_DIR}"

if ! [ -f "${FONTS_DIR}/fs-tahoma-8px.ttf" ]; then
	cp "./resources/fs-tahoma-8px.ttf" "${FONTS_DIR}/fs-tahoma-8px.ttf"
fi

fc-cache -fv
