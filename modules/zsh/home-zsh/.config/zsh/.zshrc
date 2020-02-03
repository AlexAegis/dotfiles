#!/usr/bin/zsh
# zsh startup file

echo Starting zsh

. $ZDOTDIR/.zshrc.default
. $ZDOTDIR/.zsh-plugins.antibody.sh
. $ZDOTDIR/.zshrc.p10k

# TODO: history plugin https://github.com/ohmyzsh/ohmyzsh/blob/master/lib/history.zsh
