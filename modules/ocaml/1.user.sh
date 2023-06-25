#!/bin/sh

opam init --no

# https://ocaml.org/docs/up-and-running#installing-the-ocaml-platform-tools
opam install --yes dune merlin ocaml-lsp-server odoc ocamlformat utop dune-release
