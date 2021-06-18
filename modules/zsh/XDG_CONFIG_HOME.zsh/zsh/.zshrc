#!/bin/zsh
# zsh startup file
# start profiling
if [ "$BENCH" ]; then
	zmodload zsh/zprof
fi
# ZSH_TMUX_AUTOSTART=true
# ZSH_TMUX_FIXTERM=true

export LOADENV_RC_EXTS="sh zsh" # used by loadenv
# TODO: The compinit file generates errors on debian buster
# Custom Compdef
fpath=($XDG_CONFIG_HOME/zsh/compdef $fpath)
unsetopt PROMPT_SP
# Enable Powerlevel10k instant prompt. Should stay close to the top
# of ~/.config/zsh/.zshrc. Initialization code that may require console input
# (password prompts, [y/n] confirmations, etc.) must go above this block,
# everything else may go below.
if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  . "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi

# the fzf plugin needs it, but to overwrite everything else, .profile has
# to be loaded after the plugins.
# the fzf plugin is no longer used but this is left here as reference
# . ~/.config/environment.d/51-fzf.conf
# Order matters so you can overwrite stuff that the plugins load
# omzsh throws some unnecessary warnings unless comfix is disabled
# while it's loading
[ -f "$ZDOTDIR/.zsh-plugins.antibody.sh" ] && \
	ZSH_DISABLE_COMPFIX=true . "$ZDOTDIR/.zsh-plugins.antibody.sh"
# ! YOUR ENVIRONMENT LOADS AT THIS POINT! ANYTHING BEFORE IT MUST WORK WITHOUT
# ! IT OR PRECOMPILE IT IN THE .zshenv FILE LIKE THE $XDG_CONFIG_HOME AND
# ! XDG_CACHE_HOME FILES ARE (and anything in the 20-zshenv.conf file)!
[ -f "$HOME/.profile" ] && QUIET=1 . "$HOME/.profile"


[ -f "$ZDOTDIR/.zshrc.default" ] && . "$ZDOTDIR/.zshrc.default"
[ -f "$ZDOTDIR/.zshrc.local" ] && . "$ZDOTDIR/.zshrc.local"


## zsh config

export KEYTIMEOUT=1 # esc has a default timeout of 0.4 sec which is annoying in vi mode

# TODO: history plugin https://github.com/ohmyzsh/ohmyzsh/blob/master/lib/history.zsh

POWERLEVEL9K_ALWAYS_SHOW_USER=true

[ -f "$ZDOTDIR/.p10k.zsh" ] && . "$ZDOTDIR/.p10k.zsh"

# CUSTOM_CONTEXT_RUNE='VVX'
POWERLEVEL9K_CONTEXT_TEMPLATE="`if command -v runic &>/dev/null; then
	runic ${CUSTOM_CONTEXT_RUNE:-$USER}
else echo ${CUSTOM_CONTEXT_RUNE:-$USER}; fi
`"

typeset -g POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(
  context dir vcs
)

unset POWERLEVEL9K_CONTEXT_DEFAULT_CONTENT_EXPANSION
## Base config
# Enable vi mode
bindkey -v

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
if [ "$BENCH" ]; then
	zprof
fi

# tmux
#if [[ -n ${TMUX} && -n ${commands[tmux]} ]];then
#	case $(tmux showenv TERM 2>/dev/null) in
#		*256color) ;&
#			TERM=fbterm)
#			TERM=screen-256color ;;
#		*)
#			TERM=screen
#		esac
#fi

#if command -v tmux &> /dev/null && [ -n "$PS1" ] \
#	&& [[ ! "$TERM" =~ screen ]] \
#	&& [[ ! "$TERM" =~ tmux ]] \
#	&& [ -z "$TMUX" ]; then
#	tmux a -t default || exec tmux new -s default && exit;
#fi
