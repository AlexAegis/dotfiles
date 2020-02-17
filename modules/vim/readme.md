# Vim

Autoload .vimrc securely

```vim
set secure
if filereadable(expand(printf('%s/%s', getcwd(), '.vimrc')))
  exec printf('source %s/%s', getcwd(), '.vimrc')
endif
```

TODO:

```vim
set timeoutlen=1000 ttimeoutlen=0
```

TODO: VIM IntelliSense / LSP
<https://github.com/neoclide/coc.nvim>
