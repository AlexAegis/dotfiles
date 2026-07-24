#!/bin/sh

BETTER_BLUR_DX_DIR="$XDG_DATA_HOME/kwin-effects-better-blur-dx"

if ! [ -d "$BETTER_BLUR_DX_DIR/.git" ]; then
	git clone https://github.com/xarblu/kwin-effects-better-blur-dx \
		"$BETTER_BLUR_DX_DIR"
else
	(
		cd "$BETTER_BLUR_DX_DIR" || exit 1
		git pull
	)
fi

cd "$BETTER_BLUR_DX_DIR" || exit 1

rm -rf build
mkdir -p build
cd build || exit 1

# Wayland is the default, pass -DBBDX_X11=ON for a KWin X11 build
cmake .. -DCMAKE_INSTALL_PREFIX=/usr
make -j"$(nproc)"
