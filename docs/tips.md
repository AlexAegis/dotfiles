# Tips

## [Check if program exists](https://stackoverflow.com/questions/592620/how-can-i-check-if-a-program-exists-from-a-bash-script)

```sh
command -v <program>
```

Don't use `which`, it's relatively expensive

## Check what package a command belongs to

With pacman:

```sh
pacman -Qo <program>
```

## Check for insecure folders

```sh
compaudit | xargs chmod g-w,o-w
```

## [Package size](https://wiki.archlinux.org/index.php/Pacman/Tips_and_tricks#With_size)

```sh
LC_ALL=C pacman -Qi | awk '/^Name/{name=$3} /^Installed Size/{print $4$5, name}' | sort -h
```

```sh
pacman -Syu expac
expac -S -H M '%k\t%n'
```

```sh
pacman -Quq|xargs expac -S -H M '%k\t%n' | sort -sh
```

Folder size

```sh
du -sh .
```
