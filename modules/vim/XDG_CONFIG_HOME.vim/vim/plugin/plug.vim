" Plug plugins and their configs

call plug#begin('$XDG_CONFIG_HOME/vim/bundle')
Plug 'junegunn/vim-plug'
Plug 'tpope/vim-fugitive'
Plug 'scrooloose/nerdtree', { 'on':  'NERDTreeToggle' }
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'
Plug 'terryma/vim-multiple-cursors'
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
