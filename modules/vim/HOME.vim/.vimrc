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

" TODO: Make non printable characters show in only visual mode (always)
" How non-printable things are shown
set list                              " show non-print characters,...
set listchars=trail:⋅,nbsp:⋅,tab:▷⋅   " for tabs and trailing spaces
set number                            " show line number for current line,...
set relativenumber                    " and relative line numbers for others
set cursorline                        " highlight the line with the cursor
set wildmenu                          " use command-line completion menu,...
set wildmode=longest:full             " with wildmode
set autoindent                        " enable auto-indentation

" How Search behaves
"set ignorecase                        " assume patterns are case insensitive,...
set smartcase                         " ...when lowercase-only is used
set incsearch                         " start searching as you type

" How IO behave
set ttyfast                           " assume the terminal is fast
set mouse=a                           " enable mouse in all modes



" Tweak how Gvim looks
set guifont=Inconsolata-g\ Medium\ 11 " set font in gvim
set guioptions-=T                     " no toolbar in gvim
set guioptions-=m                     " no menubar in gvim
set guioptions-=r                     " no right scrollbar in gvim
set guioptions-=L                     " no left scrollbar when v.split in gvim


" TODO: Check this out: https://dev.to/gaveen/my-vim-story-594d
