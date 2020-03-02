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