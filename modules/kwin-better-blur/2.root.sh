#!/bin/sh

KWIN_FORCEBLUR_DIR="$XDG_DATA_HOME/kwin-effects-forceblur"

cd "$KWIN_FORCEBLUR_DIR/build" || exit 1
make install
