" Plugins

let mapleader=' '

call plug#begin('~/.vim/bundle')
Plug 'junegunn/vim-plug'
Plug 'tpope/vim-fugitive'
Plug 'scrooloose/nerdtree', { 'on':  'NERDTreeToggle' }
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'
Plug 'terryma/vim-multiple-cursors'
call plug#end()

" Main settings
set number relativenumber

set splitbelow splitright

" PlugInstall
" PlugUpdate
" TODO Autoupdate on launch?
" TODO Autoexit after successful update

" How Tab behaves
set tabstop=4            " number of spaces used as tab for file
set softtabstop=4        " number of spaces used as tab for editing
set shiftwidth=4         " number of spaces used to autoindent
set expandtab            " expand tabs into spaces
set smarttab             " smart tabulation and backspace
set bs=indent,eol,start  " allow backspacing over everything

" Hungry Delete. [source](https://www.reddit.com/r/vim/comments/7gqowu)
inoremap <s-bs> <bs>			" shift + backspace to do regular backspace
inoremap <silent><expr><bs>
  \ (&indentexpr isnot '' ? &indentkeys : &cinkeys) =~? '!\^F' &&
  \ &backspace =~? '.*eol\&.*start\&.*indent\&' &&
  \ !search('\S','nbW',line('.')) ? (col('.') != 1 ? "\<C-U>" : "") .
  \ "\<bs>" . (getline(line('.')-1) =~ '\S' ? "" : "\<C-F>") : "\<bs>"
