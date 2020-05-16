# ZSH

[Startup files](http://zsh.sourceforge.net/Intro/intro_3.html)

## Benchmark startup speed

```zsh
time zsh -i -c exit
```

## Unused plugins

- robbyrussell/oh-my-zsh path:plugins/vi-mode

TODO: fzf should exit when backspaced beyond the original matchword.

example:

```sh
cd fo<tab>
```

prints every folder starting with fo, but when deleteing one, the ones starting with f does not appear. fzf should be restarted with the new data in this case
