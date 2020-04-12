" Plug plugins and their configs

call plug#begin('$XDG_CONFIG_HOME/vim/bundle')
Plug 'junegunn/vim-plug'
Plug 'junegunn/fzf'
Plug 'junegunn/fzf.vim'
Plug 'tpope/vim-fugitive'
Plug 'scrooloose/nerdtree', { 'on':  'NERDTreeToggle' }
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'
Plug 'terryma/vim-multiple-cursors'
Plug 'flazz/vim-colorschemes'
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


" fzf

function! s:fzf_statusline()
  " Override statusline as you like
  highlight fzf1 ctermfg=161 ctermbg=251
  highlight fzf2 ctermfg=23 ctermbg=251
  highlight fzf3 ctermfg=237 ctermbg=251
  setlocal statusline=%#fzf1#\ >\ %#fzf2#fz%#fzf3#f
endfunction

autocmd! User FzfStatusLine call <SID>fzf_statusline()
