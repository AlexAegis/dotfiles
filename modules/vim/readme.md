# [vim](https://www.vim.org/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/Vim)

## Plugins

Install plugins

### [vim-plug](https://github.com/junegunn/vim-plug)

### Fugitive

## Autoload .vimrc securely

```vim
set secure
if filereadable(expand(printf('%s/%s', getcwd(), '.vimrc')))
  exec printf('source %s/%s', getcwd(), '.vimrc')
endif
```

## Notes

TODO: <https://sevensidedmarble.com/folds-for-better-vim-configs/>

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

TODO: Set up plugins
TODO: Try hard mode <https://github.com/wikitopian/hardmode>

TODO: Remap caps lock to esc
TODO: Remap semicolon to colon in normal mode
