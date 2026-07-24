#!/bin/sh

BETTER_BLUR_DX_DIR="$XDG_DATA_HOME/kwin-effects-better-blur-dx"

# cmake --install instead of make install, make would rebuild as root and
# leave root owned artifacts in the user's tree
cmake --install "$BETTER_BLUR_DX_DIR/build" || exit 1

# The hook runs long after this module did, so it has to be told where the
# clone lives and who owns it
echo "BETTER_BLUR_DX_DIR=\"$BETTER_BLUR_DX_DIR\"
BETTER_BLUR_DX_USER=\"${SUDO_USER:-$(stat -c '%U' "$BETTER_BLUR_DX_DIR")}\"
" >/etc/kwin-better-blur-dx.conf

install -Dm 755 "./resources/kwin-better-blur-dx-rebuild" \
	"/usr/local/bin/kwin-better-blur-dx-rebuild"

mkdir -p /etc/pacman.d/hooks/

# ? This hook recompiles the effect whenever KWin is upgraded, it links
# ? against KWin internals so it breaks on every KWin update otherwise
install -Dm 644 "./resources/hooks/90-kwin-better-blur-dx.hook" \
	"/etc/pacman.d/hooks/90-kwin-better-blur-dx.hook"
