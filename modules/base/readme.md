# base

This module is an implicit dependency and is installed first every time.

It installs some packages that are considered essential for `pont`, mainly
`sudo`, `stow`, `tar` and `git`. Check the `.dependencies` file to see all of
them.

## Idea

A `base` module that is ensured to be installed everywhere without having to
be mentioned as a dependency everywhere ensures that `pont` will function.
