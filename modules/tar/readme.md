# [tar](https://www.gnu.org/software/tar/)

GNU Tar, archiving tool

> [Arch Wiki](https://wiki.archlinux.org/index.php/Archiving_and_compression)\
> [Gentoo Wiki](https://wiki.gentoo.org/wiki/Tar)

## Make a gzipped tarball out of a folder

> c: Create a TAR file\
> v: Verbose output\
> f: Specify a filename for the resulting TAR file\
> z: Compress the TAR file with GZIP

```sh
tar cvfz <filename>.tar.gz <directory>/
```

### Compressing with BZIP2

> j: Compress the TAR file with BZIP2

Use the `j` flag instead of the `z` flag, and the extension `.tar.bz2`.

## Untar gzipped tarball

> x: Extract the contents from the file specified\
> v: Verbose output\
> f: Specify a filename to uncompress\
> z: Uncompress the tarball via GZIP

```sh
tar xvfz <filename>.tar.gz
```

### Uncompressing with BZIP2

> j: Uncompress the tarball via BZIP2.

Use the `j` flag instead of the `z` flag for files with extension `.tar.bz2`.

## "Tarhash"

> This is what `pont` does to hash a folder and its contents to check for
> changes. It also excludes the file where the hash is saved of course.

```sh
tar -c "<directory>" 2> /dev/null | sha1sum
```
