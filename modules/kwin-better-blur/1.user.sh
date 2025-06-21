#!/bin/sh

KWIN_FORCEBLUR_DIR="$XDG_DATA_HOME/kwin-effects-forceblur"

if ! [ -d "$KWIN_FORCEBLUR_DIR/.git" ]; then
	git clone https://github.com/taj-ny/kwin-effects-forceblur "$KWIN_FORCEBLUR_DIR"
else
	(
		cd "$KWIN_FORCEBLUR_DIR" || exit 1
		git pull
	)
fi


if [ -d "$KWIN_FORCEBLUR_DIR/build" ]; then
	rm -rf "$KWIN_FORCEBLUR_DIR/build"
fi

cd "$KWIN_FORCEBLUR_DIR" || exit 1
mkdir build
cd build || exit 1
cmake ../ -DCMAKE_INSTALL_PREFIX=/usr
make -j
