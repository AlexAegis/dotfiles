" Common vim config

" Plug plugins and their configs

call plug#begin('$XDG_CONFIG_HOME/vim/bundle')
Plug 'junegunn/vim-plug'
Plug 'sheerun/vim-polyglot'
Plug 'vim-airline/vim-airline'
Plug 'vim-airline/vim-airline-themes'
Plug 'junegunn/fzf'
Plug 'junegunn/fzf.vim'
Plug 'tpope/vim-fugitive'
Plug 'scrooloose/nerdtree', { 'on':  'NERDTreeToggle' }
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'
Plug 'terryma/vim-multiple-cursors'
Plug 'flazz/vim-colorschemes'
Plug 'rafi/awesome-vim-colorschemes'
Plug 'rainglow/vim'
Plug 'mkarmona/colorsbox'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
" For JavaScript and TypeScript
Plug 'prettier/vim-prettier', {
  \ 'do': 'npm install',
  \ 'for': ['javascript', 'typescript', 'css', 'less', 'scss',
		\ 'json', 'graphql', 'markdown', 'vue', 'yaml', 'html'] }
call plug#end()

" Plugin Configuration
"" prettier
" Format on write
autocmd BufWritePost *.js,*.jsx,*.json,*.ts,*.tsx,*.css,*.less,
	\*.scss,*.json,*.graphql,*.md,*.yaml,*.html :Prettier:w

set undofile
" fzf

function! s:fzf_statusline()
  " Override statusline as you like
  highlight fzf1 ctermfg=161 ctermbg=251
  highlight fzf2 ctermfg=23 ctermbg=251
  highlight fzf3 ctermfg=237 ctermbg=251
  setlocal statusline=%#fzf1#\ >\ %#fzf2#fz%#fzf3#f
endfunction

autocmd! User FzfStatusLine call <SID>fzf_statusline()


syntax on

let mapleader=' '

" " Command autocomplete
" set wildmenu
" set wildmode=longest:full

" Main settings
set number relativenumber

set splitbelow splitright

" PlugInstall
" PlugUpdate
" TODO Autoupdate on launch?
" TODO Autoexit after successful update


" Hungry Delete. [source](https://www.reddit.com/r/vim/comments/7gqowu)
inoremap <s-bs> <bs>			" shift + backspace to do regular backspace
inoremap <silent><expr><bs>
  \ (&indentexpr isnot '' ? &indentkeys : &cinkeys) =~? '!\^F' &&
  \ &backspace =~? '.*eol\&.*start\&.*indent\&' &&
  \ !search('\S','nbW',line('.')) ? (col('.') != 1 ? "\<C-U>" : "") .
  \ "\<bs>" . (getline(line('.')-1) =~ '\S' ? "" : "\<C-F>") : "\<bs>"

nnoremap <leader>v :call Reload()<CR>

" hi Normal ctermbg=235 ctermfg=250 guibg=#262626 guifg=#bcbcbc cterm=NONE gui=NONE
" hi Visual ctermbg=110 ctermfg=235 guibg=#8fafd7 guifg=#262626 cterm=NONE gui=NONE
" hi SpecialKey ctermbg=NONE ctermfg=NONE guibg=#8fafd744 guifg=NONE cterm=NONE gui=NONE
colorscheme apprentice

 hi Normal ctermbg=none guibg=NONE

set encoding=utf-8
" TODO: Make non printable characters show in only visual mode (always)
" How non-printable things are shown
set listchars=eol:$,tab:▰▱,nbsp:⋅,trail:~,extends:>,precedes:<
" set list                              " show non-print characters,...
set number                            " show line number for current line,...
set relativenumber                    " and relative line numbers for others
set cursorline                        " highlight the line with the cursor
set wildmenu                          " use command-line completion menu,...
set wildmode=longest:full             " with wildmode
set wildchar=<Tab>
" Tab
set autoindent                        " enable auto-indentation
set tabstop=4            " number of spaces used as tab for file
set softtabstop=4        " number of spaces used as tab for editing
set shiftwidth=4         " number of spaces used to autoindent
" set expandtab            " expand tabs into spaces
set smarttab             " smart tabulation and backspace
set bs=indent,eol,start  " allow backspacing over everything

set termguicolors
let &t_8f = "\e[38;2;%lu;%lu;%lum"
let &t_8b = "\e[48;2;%lu;%lu;%lum"

" Move a line of text using TODO: Respect indentation
nmap <silent><M-j> mz:m+<cr>`z
nmap <silent><M-k> mz:m-2<cr>`z
vmap <silent><M-j> :m'>+<cr>`<my`>mzgv`yo`z
vmap <silent><M-k> :m'<-2<cr>`>my`<mzgv`yo`z
" Clone a line of text
nmap <silent><S-M-j> YP<cr>
nmap <silent><S-M-k> Yp2k<cr>

" How Search behaves
"set ignorecase                        " assume patterns are case insensitive,...
set smartcase                         " ...when lowercase-only is used
set incsearch                         " start searching as you type

" How IO behave
set ttyfast                           " assume the terminal is fast
set mouse=a                           " enable mouse in all modes



" Tweak how Gvim looks
set guifont=Iosevka\ 11 " set font in gvim
set guioptions-=T                     " no toolbar in gvim
set guioptions-=m                     " no menubar in gvim
set guioptions-=r                     " no right scrollbar in gvim
set guioptions-=L                     " no left scrollbar when v.split in gvim


" TODO: Check this out: https://dev.to/gaveen/my-vim-story-594d


function! InsertTabWrapper()
  if pumvisible()
    return "\<c-n>"
  endif
  let col = col('.') - 1
  if !col || getline('.')[col - 1] !~ '\k'
    return "\<tab>"
  else
    return "\<c-x>\<c-o>"
  endif
endfunction
inoremap <expr><tab> InsertTabWrapper()
inoremap <expr><s-tab> pumvisible()?"\<c-p>":"\<c-d>"
