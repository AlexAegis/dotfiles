#!/usr/bin/zsh
# zsh startup file
# start profiling
# zmodload zsh/zprof

# Enable Powerlevel10k instant prompt. Should stay close to the top of ~/.config/zsh/.zshrc.
# Initialization code that may require console input (password prompts, [y/n]
# confirmations, etc.) must go above this block, everything else may go below.
if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  source "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi

echo Starting zsh

[[ ! -f $HOME/.profile ]] || source $HOME/.profile
[[ ! -f $ZDOTDIR/.zshrc.default ]] || source $ZDOTDIR/.zshrc.default
[[ ! -f $ZDOTDIR/.zsh-plugins.antibody.sh ]] || source $ZDOTDIR/.zsh-plugins.antibody.sh

# Load any optional `sh` and `zsh` rc files from the .rc folder

. $HOME/bin/sourcein "$HOME/.rc/" ".*.[sh|zsh]"

# TODO: history plugin https://github.com/ohmyzsh/ohmyzsh/blob/master/lib/history.zsh

# To customize prompt, run `p10k configure` or edit ~/.config/zsh/.p10k.zsh.
[[ ! -f $ZDOTDIR/.p10k.zsh ]] || source $ZDOTDIR/.p10k.zsh

## Plugin configurations

# zsh-users/zsh-autosuggestions
# https://github.com/zsh-users/zsh-autosuggestions
export ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=#444"
export ZSH_AUTOSUGGEST_STRATEGY=(completion history)
export ZSH_AUTOSUGGEST_BUFFER_MAX_SIZE=20
export ZSH_AUTOSUGGEST_USE_ASYNC=1

# direnv hook
if command -v direnv > /dev/null 2>&1; then
	eval "$(direnv hook zsh)"
fi

# end profiling
# zprof
