#!/bin/sh

# Add the default id_rsa key to keychain
ssh-add -q < /dev/null
# To add other keys aswell
# ssh-add -q ~/.ssh/key1 ~/.ssh/key2 ~/.ssh/key3 < /dev/null
