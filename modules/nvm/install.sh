#!/bin/bash
. /usr/share/nvm/init-nvm.sh
nvm install --lts # latest LTS
nvm install node # latest
nvm alias default node # always default to latest node
nvm use node # use latest node
# install some default global packages                                                                                                        1 ✘  19:22:34
npm install -g npm-check-updates # version checker
