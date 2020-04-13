"             _
"  _ ____   _(_)_ __ ___
" | '_ \ \ / / | '_ ` _ \
" | | | \ V /| | | | | | |
" |_| |_|\_/ |_|_| |_| |_|
"

set runtimepath=$XDG_CONFIG_HOME/nvim,$XDG_CONFIG_HOME/nvim/after
set directory=$XDG_CACHE_HOME/nvim/swap//,.,~/tmp,/var/tmp,/tmp
set backupdir=$XDG_CONFIG_HOME/nvim/backup//,.,~/tmp,~/
set viminfo=n$XDG_CACHE_HOME/nvim/viminfo
set undodir=$XDG_CONFIG_HOME/nvim/undo//,.
let $MYVIMRC="$XDG_CONFIG_HOME/nvim/init.vim"

let &packpath=&runtimepath
source $XDG_CONFIG_HOME/vim/init.vim
