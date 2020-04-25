"        _
" __   _(_)_ __ ___  _ __ ___
" \ \ / / | '_ ` _ \| '__/ __|
"  \ V /| | | | | | | | | (__
" (_)_/ |_|_| |_| |_|_|  \___|
"
" Author: Győri Sándor (AlexAegis) <alexaegis@gmail.com>
""""""""""
"  Plug  "
""""""""""
"" Autoload if missing
if empty(glob('$XDG_CONFIG_HOME/vim/autoload/plug.vim'))
  silent !curl -fLo $XDG_CONFIG_HOME/vim/autoload/plug.vim --create-dirs
    \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
  autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif
"" Load plugins
call plug#begin('$XDG_CONFIG_HOME/vim/bundle')
Plug 'junegunn/vim-plug'
Plug 'sheerun/vim-polyglot'
Plug 'vim-airline/vim-airline'
Plug 'vim-airline/vim-airline-themes'
Plug 'junegunn/fzf'
Plug 'junegunn/fzf.vim'
Plug 'tpope/vim-fugitive'
Plug 'airblade/vim-gitgutter'
Plug 'scrooloose/nerdtree', { 'on':  'NERDTreeToggle' }
Plug 'Xuyuanp/nerdtree-git-plugin'
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'
Plug 'mg979/vim-visual-multi', { 'branch': 'master' }
Plug 'flazz/vim-colorschemes'
Plug 'rafi/awesome-vim-colorschemes'
Plug 'chuling/vim-equinusocio-material'
Plug 'sainnhe/gruvbox-material'
Plug 'rainglow/vim'
Plug 'mkarmona/colorsbox'
" Plug 'jiangmiao/auto-pairs' " Causes problems with coc enter accept
" Plug 'alvan/vim-closetag'
Plug 'doums/coBra'
" Dev
" Plug 'vim-scripts/Conque-GDB'
Plug 'majutsushi/tagbar'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
"" HTML
Plug 'mattn/emmet-vim', {
  \ 'for': ['javascript', 'typescript', 'html', 'xhtml', 'xml'] }
"" Typescript
Plug 'prettier/vim-prettier', {
  \ 'do': 'npm install -g prettier',
  \ 'for': ['javascript', 'typescript', 'css', 'less', 'scss',
		\ 'json', 'graphql', 'markdown', 'vue', 'yaml', 'html'] }
"" Rust
Plug 'rust-lang/rust.vim'
call plug#end()

"" Plugin Configuration
""" scrooloose/nerdtree
let g:NERDTreeHijackNetrw = 1	" On by default, just to be sure
autocmd VimEnter * NERDTree		" Autotart NERDTree
autocmd VimEnter * wincmd p		" Go to previous (last accessed) window.
" Store the bookmarks file
let NERDTreeBookmarksFile=expand("$XDG_CONFIG_HOME/vim/nerd_tree_bookmarks")
" Show the bookmarks table on startup
let NERDTreeShowBookmarks=1

" Exit NERDTree if it's the last buffer
autocmd bufenter * if (winnr("$") == 1 && exists("b:NERDTree") &&
	\ b:NERDTree.isTabTree()) | q | endif
""" Xuyuanp/nerdtree-git-plugin
let g:NERDTreeIndicatorMapCustom = {
    \ "Modified"  : "✹",
    \ "Staged"    : "✚",
    \ "Untracked" : "✭",
    \ "Renamed"   : "➜",
    \ "Unmerged"  : "═",
    \ "Deleted"   : "✖",
    \ "Dirty"     : "✗",
    \ "Clean"     : "✔︎",
    \ 'Ignored'   : '☒',
    \ "Unknown"   : "?"
    \ }
""" prettier/vim-prettier
let g:prettier#exec_cmd_path = "prettier"
let g:prettier#autoformat = 1
""" rust-lang/rust.vim
let g:rustfmt_autosave = 1
:nnoremap <F6> :RustRun<CR>
" Format on write
" *.md, markdown format seems bad, removes entire buffer on format
autocmd BufWritePost *.js,*.jsx,*.json,*.ts,*.tsx,*.css,*.less,
	\*.scss,*.json,*.graphql,*.yaml,*.html :Prettier:w
""" vim-airline/vim-airline
" Disable the default mode show
set noshowmode
" bottom cmd height can be set with cmdheight=1
""" sainnhe/gruvbox-material

" important!!
set termguicolors

" for dark version
set background=dark

" for light version
" set background=light

" set contrast
" this configuration option should be placed before `colorscheme gruvbox-material`
" available values: 'hard', 'medium'(default), 'soft'
let g:gruvbox_material_background = 'hard'
let g:gruvbox_material_transparent_background = 1

" For airline
let g:airline_theme = 'gruvbox_material'

colorscheme gruvbox-material

""" airblade/vim-gitgutter
let g:gitgutter_sign_added = '█'
let g:gitgutter_sign_modified = '█'
let g:gitgutter_sign_removed = '█'
let g:gitgutter_sign_removed_first_line = '█'
let g:gitgutter_sign_modified_removed = '█'
let g:gitgutter_highlight_linenrs = 1
""" neoclide/coc.nvim
" Remap for rename current word
nmap <leader>rn <Plug>(coc-rename)
nmap <F2> <Plug>(coc-rename)

" CocInstall coc-rust-analyzer
" CocInstall coc-emmet

" TextEdit might fail if hidden is not set.
set hidden

" Some servers have issues with backup files, see #649.
set nobackup
set nowritebackup

" Having longer updatetime (default is 4000 ms = 4 s) leads to noticeable
" delays and poor user experience.
set updatetime=300

" Don't pass messages to |ins-completion-menu|.
set shortmess+=c

" Always show the signcolumn, otherwise it would shift the text each time
" diagnostics appear/become resolved.
set signcolumn=yes

" Use tab for trigger completion with characters ahead and navigate.
" NOTE: Use command ':verbose imap <tab>' to make sure tab is not mapped by
" other plugin before putting this into your config.
inoremap <silent><expr> <TAB>
	\ pumvisible() ? "\<C-n>" :
	\ <SID>check_back_space() ? "\<TAB>" :
	\ coc#refresh()
inoremap <expr><S-TAB> pumvisible() ? "\<C-p>" : "\<C-h>"

function! s:check_back_space() abort
	let col = col('.') - 1
	return !col || getline('.')[col - 1]  =~# '\s'
endfunction

" Accept completion with Enter.
inoremap <silent><expr> <cr> pumvisible() ? coc#_select_confirm() :
	\"\<C-g>u\<CR>\<c-r>=coc#on_enter()\<CR>"

" Use <c-space> to trigger completion.
inoremap <silent><expr> <c-space> coc#refresh()

" Use `[g` and `]g` to navigate diagnostics
nmap <silent> [g <Plug>(coc-diagnostic-prev)
nmap <silent> ]g <Plug>(coc-diagnostic-next)

" GoTo code navigation.
nmap <silent> gd <Plug>(coc-definition)
nmap <silent> gy <Plug>(coc-type-definition)
nmap <silent> gi <Plug>(coc-implementation)
nmap <silent> gr <Plug>(coc-references)

" Use K to show documentation in preview window.
nnoremap <silent> K :call <SID>show_documentation()<CR>

function! s:show_documentation()
  if (index(['vim','help'], &filetype) >= 0)
    execute 'h '.expand('<cword>')
  else
    call CocAction('doHover')
  endif
endfunction

" Highlight the symbol and its references when holding the cursor.
autocmd CursorHold * silent call CocActionAsync('highlight')

" Symbol renaming.
nmap <leader>rn <Plug>(coc-rename)

" Formatting selected code.
xmap <leader>f  <Plug>(coc-format-selected)
nmap <leader>f  <Plug>(coc-format-selected)

augroup mygroup
  autocmd!
  " Setup formatexpr specified filetype(s).
  autocmd FileType typescript,json setl formatexpr=CocAction('formatSelected')
  " Update signature help on jump placeholder.
  autocmd User CocJumpPlaceholder call CocActionAsync('showSignatureHelp')
augroup end

" Applying codeAction to the selected region.
" Example: `<leader>aap` for current paragraph
xmap <leader>a  <Plug>(coc-codeaction-selected)
nmap <leader>a  <Plug>(coc-codeaction-selected)

" Remap keys for applying codeAction to the current line.
nmap <leader>ac  <Plug>(coc-codeaction)
" Apply AutoFix to problem on the current line.
nmap <leader>qf  <Plug>(coc-fix-current)

" Introduce function text object
" NOTE: Requires 'textDocument.documentSymbol' support from the language server.
xmap if <Plug>(coc-funcobj-i)
xmap af <Plug>(coc-funcobj-a)
omap if <Plug>(coc-funcobj-i)
omap af <Plug>(coc-funcobj-a)

" Use <TAB> for selections ranges.
" NOTE: Requires 'textDocument/selectionRange' support from the language server.
" coc-tsserver, coc-python are the examples of servers that support it.
nmap <silent> <TAB> <Plug>(coc-range-select)
xmap <silent> <TAB> <Plug>(coc-range-select)

" Add `:Format` command to format current buffer.
command! -nargs=0 Format :call CocAction('format')

" Add `:Fold` command to fold current buffer.
command! -nargs=? Fold :call     CocAction('fold', <f-args>)

" Add `:OR` command for organize imports of the current buffer.
command! -nargs=0 OR   :call     CocAction('runCommand', 'editor.action.organizeImport')

" Add (Neo)Vim's native statusline support.
" NOTE: Please see `:h coc-status` for integrations with external plugins that
" provide custom statusline: lightline.vim, vim-airline.
set statusline^=%{coc#status()}%{get(b:,'coc_current_function','')}
"" coc extensions
"""
" TODO: let g:coc_global_extensions^='coc-emmet'
"""""""""""""
"  Folding  "
"""""""""""""
"" Common
" set foldmethod=syntax
" set foldlevelstart=99
"set nofoldenable	" No fold on launch
"" Vim
" Autofolding .vimrc
" see http://vimcasts.org/episodes/writing-a-custom-fold-expression/
""" Foldlevel definition
function! VimFolds(lnum)
  let s:thisline = getline(a:lnum)
  if match(s:thisline, '^"" ') >= 0
    return '>2'
  endif
  if match(s:thisline, '^""" ') >= 0
    return '>3'
  endif
  let s:two_following_lines = 0
  if line(a:lnum) + 2 <= line('$')
    let s:line_1_after = getline(a:lnum+1)
    let s:line_2_after = getline(a:lnum+2)
    let s:two_following_lines = 1
  endif
  if !s:two_following_lines
      return '='
    endif
  else
    if (match(s:thisline, '^"""""') >= 0) &&
       \ (match(s:line_1_after, '^"  ') >= 0) &&
       \ (match(s:line_2_after, '^""""') >= 0)
      return '>1'
    else
      return '='
    endif
  endif
endfunction
""" Foldtext definition
function! VimFoldText()
  " handle special case of normal comment first
  let s:info = '('.string(v:foldend-v:foldstart).' l)'
  if v:foldlevel == 1
    let s:line = ' ◇ '.getline(v:foldstart+1)[3:-2]
  elseif v:foldlevel == 2
    let s:line = '   ●  '.getline(v:foldstart)[3:]
  elseif v:foldlevel == 3
    let s:line = '     ▪ '.getline(v:foldstart)[4:]
  endif
  if strwidth(s:line) > 80 - len(s:info) - 3
    return s:line[:79-len(s:info)-3+len(s:line)-strwidth(s:line)].'...'.s:info
  else
    return s:line.repeat(' ', 80 - strwidth(s:line) - len(s:info)).s:info
  endif
endfunction
" set foldsettings automatically for vim files
augroup fold_vimrc
  autocmd!
  autocmd FileType vim
                   \ setlocal foldmethod=expr |
                   \ setlocal foldexpr=VimFolds(v:lnum) |
                   \ setlocal foldtext=VimFoldText() |
                   \ set foldcolumn=2 foldminlines=2
augroup END
"" Markdown
let g:markdown_folding = 0

"" Keybindings
" Using leader space (double space)
vnoremap <leader><space> zf
nnoremap <silent> <leader><space> @=(foldlevel('.')?'za':"  ")<CR>
" Using double click
noremap <2-LeftMouse> za
""""""""""""
"  Common  "
""""""""""""
"" General
let mapleader=' '
set undofile
set updatetime=100				" Default is 4000
set number relativenumber		" Gutter numbers
set splitbelow splitright		" Order of window splitting
"" Filetypes & highlighting
filetype plugin indent on
syntax on
"""""""""""""""""
"  Keybindings  "
"""""""""""""""""
"" Hungry Delete.
" [source](https://www.reddit.com/r/vim/comments/7gqowu)
inoremap <s-bs> <bs>			" shift + backspace to do regular backspace
inoremap <silent><expr><bs>
  \ (&indentexpr isnot '' ? &indentkeys : &cinkeys) =~? '!\^F' &&
  \ &backspace =~? '.*eol\&.*start\&.*indent\&' &&
  \ !search('\S','nbW',line('.')) ? (col('.') != 1 ? "\<C-U>" : "") .
  \ "\<bs>" . (getline(line('.')-1) =~ '\S' ? "" : "\<C-F>") : "\<bs>"
"" Terminal
:tnoremap <Esc> <C-\><C-n>
" fzf

" function! s:fzf_statusline()
"   " Override statusline as you like
"   highlight fzf1 ctermfg=161 ctermbg=251
"   highlight fzf2 ctermfg=23 ctermbg=251
"   highlight fzf3 ctermfg=237 ctermbg=251
"   setlocal statusline=%#fzf1#\ >\ %#fzf2#fz%#fzf3#f
" endfunction
"
" autocmd! User FzfStatusLine call <SID>fzf_statusline()






" " Command autocomplete
" set wildmenu
" set wildmode=longest:full

" Main settings


" PlugInstall
" PlugUpdate
" TODO Autoupdate on launch?
" TODO Autoexit after successful update



nnoremap <leader>v :call Reload()<CR>

" hi Normal ctermbg=235 ctermfg=250 guibg=#262626 guifg=#bcbcbc cterm=NONE gui=NONE
" hi Visual ctermbg=110 ctermfg=235 guibg=#8fafd7 guifg=#262626 cterm=NONE gui=NONE
" hi SpecialKey ctermbg=NONE ctermfg=NONE guibg=#8fafd744 guifg=NONE cterm=NONE gui=NONE
" colorscheme apprentice

hi Normal ctermbg=none guibg=NONE

set encoding=utf-8
" TODO: Make non printable characters show in only visual mode (always)
" How non-printable things are shown
" set listchars=eol:$,tab:▰▱,nbsp:⋅,trail:~,extends:>,precedes:<
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

"let &t_8f = "\e[38;2;%lu;%lu;%lum"
"let &t_8b = "\e[48;2;%lu;%lu;%lum"

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
