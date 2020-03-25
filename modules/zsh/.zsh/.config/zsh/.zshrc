#!/bin/zsh
# zsh startup file
# start profiling
[ "$BENCH" ] && zmodload zsh/zprof

# Custom Compdef
fpath=(~/.config/zsh/compdef $fpath)
unsetopt PROMPT_SP
# Enable Powerlevel10k instant prompt. Should stay close to the top
# of ~/.config/zsh/.zshrc. Initialization code that may require console input
# (password prompts, [y/n] confirmations, etc.) must go above this block,
# everything else may go below.
if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  . "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi

# Order matters so you can overwrite stuff that the plugins load
[ -f "$ZDOTDIR/.zsh-plugins.antibody.sh" ] && . "$ZDOTDIR/.zsh-plugins.antibody.sh"

[ -f "$ZDOTDIR/.zshrc.default" ] && . "$ZDOTDIR/.zshrc.default"
[ -f "$ZDOTDIR/.zshrc.local" ] && . "$ZDOTDIR/.zshrc.local"
export LOADENV_RC_EXTS="sh zsh" # used by loadenv
[ -f "$HOME/.profile" ] && QUIET=1 . "$HOME/.profile"

## zsh config

export KEYTIMEOUT=1 # esc has a default timeout of 0.4 sec which is annoying in vi mode

# TODO: history plugin https://github.com/ohmyzsh/ohmyzsh/blob/master/lib/history.zsh

# To customize prompt, run `p10k configure` or edit ~/.config/zsh/.p10k.zsh.
[ -f "$ZDOTDIR/.p10k.zsh" ] && . "$ZDOTDIR/.p10k.zsh"

## Base config

# TODO: https://raw.githubusercontent.com/romkatv/zsh4humans/master/.zshrc

# autocomplete should include hidden files
setopt globdots
# but not current and parent directories
zstyle ':completion:*' special-dirs false

## Plugin configurations

# Powerline10k settings
# TODO: bugged and removes autocomplete suggestions
export POWERLEVEL9K_EXPERIMENTAL_TIME_REALTIME=false

# zsh-users/zsh-autosuggestions
# https://github.com/zsh-users/zsh-autosuggestions
export ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=#444"
export ZSH_AUTOSUGGEST_STRATEGY=(completion history)
export ZSH_AUTOSUGGEST_BUFFER_MAX_SIZE=20
export ZSH_AUTOSUGGEST_USE_ASYNC=1

# direnv hook
if [ command -v direnv > /dev/null 2>&1 ]; then
	eval "$(direnv hook zsh)"
fi

# end profiling
[ "$BENCH" ] && zprof
