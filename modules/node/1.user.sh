#!/bin/sh

# The reason why this is duplicated because 50-fnm-env.sh requires fnm to run
# And upon installing this module, 50-fnm-env.sh in e.user.sh does nothing.
# But once it get's here, fnm will be installed and it can!
# But it's useful to have the env script here so other modules that need node
# can rely on it being enabled by fnm

fnm install lts-latest
fnm install 16
fnm use 16
