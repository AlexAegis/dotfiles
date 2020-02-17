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

TODO:
Explore if pasting would apply autoindenting and autoquoting
Quick way do disable any autoformatting during typing
because many terminals just simulate paste by typing in
the content of the clipboard (like in WSL)
(maybe a quick tip when vim notices you typing incredibly fast)
