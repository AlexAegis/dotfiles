# [vim](https://www.vim.org/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/Vim)

## Plugins

Install plugins

### [vim-plug](https://github.com/junegunn/vim-plug)

### [wal](../wal)

Wal is an optional dependency of this `vim` module. If installed, and the
`wal` variable is set, then `vim` will use the global `wal` theme.

### Fugitive

## Autoload .vimrc securely

```vim
set secure
if filereadable(expand(printf('%s/%s', getcwd(), '.vimrc')))
  exec printf('source %s/%s', getcwd(), '.vimrc')
endif
```

## Notes

- If syntax highlighting is not working it's likely because runtimepath was
  not set correctly

TODO: <https://sevensidedmarble.com/folds-for-better-vim-configs/>

TODO:

```vim
set timeoutlen=1000 ttimeoutlen=0
```

TODO: VIM IntelliSense / LSP
<https://github.com/neoclide/coc.nvim>
<https://github.com/neoclide/coc.nvim/wiki/Using-coc-extensions>
Setup html js ts extensions, check your vscode extensions

- automatically rename tags without deleting the arguments

TODO: Ctrl + ` to enter end exit terminal mode. and also open and close
its buffer. For a dedicated buffer at the bottom
<https://github.com/neoclide/coc-prettier>

TODO: <https://github.com/camspiers/animate.vim> config, easing etc

TODO: Config airline

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

TODO: Properly separate viminfo of vim and nvim
