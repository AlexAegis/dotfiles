" Plugins

call plug#begin('~/.vim/bundle')
Plug 'junegunn/vim-plug'
Plug 'tpope/vim-fugitive'
Plug 'scrooloose/nerdtree', { 'on':  'NERDTreeToggle' }
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'
Plug 'terryma/vim-multiple-cursors'
call plug#end()

" PlugInstall
" PlugUpdate
" TODO Autoupdate on launch?
" TODO Autoexit after successful update
