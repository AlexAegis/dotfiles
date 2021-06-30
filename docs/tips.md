# Tips

## [Check if program exists](https://stackoverflow.com/questions/592620)

> Don't forget to route it's output to `/dev/null` when using this in an
> `if` statement.

```sh
command -v <program> > /dev/null
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

Get the home folder of the invoker regardless of sudo

```sh
invoker_home=$(getent passwd "${SUDO_USER-$USER}" | cut -d: -f6)
```
