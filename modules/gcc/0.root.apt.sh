#!/bin/sh

# TODO: Split this into multiple modules,
# clang libtool-bin m4 autotools-dev automake gettext
# were only added to build nvim

apt install -y gcc llvm libclang-dev clang libtool-bin \
	m4 autotools-dev automake gettext
