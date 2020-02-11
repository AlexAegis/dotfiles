# nvm

in zsh nvm is sourced from a plugin but other shells needs to source it.

TODO: Check root nvm, npm and node access <https://stackoverflow.com/questions/21215059/cant-use-nvm-from-root-or-sudo>
Check the symlink solution (maybe use stow)
sudo ln -s "$NVM_DIR/versions/node/$(nvm version)/bin/node" "/usr/local/bin/node"
sudo ln -s "$NVM_DIR/versions/node/$(nvm version)/bin/npm" "/usr/local/bin/npm"
