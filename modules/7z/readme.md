# [7z](https://www.7-zip.org/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/P7zip)\
> [Gentoo Wiki](https://wiki.gentoo.org/wiki/P7zip)

## Creating protected archives

> You will be prompted for a password if not passed

```sh
7z a -p'<optional_password>' -mhe=on <target>.7z <...files>
```

## Usage

### Unzipping

```sh
7z x example.zip -oexample
```
