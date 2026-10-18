# [Rust](https://www.rust-lang.org/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/rust)\
> [Gentoo Wiki](https://wiki.gentoo.org/wiki/Project:Rust)

This module installs `rustup` with it the stable toolchain and some
must have crates such as [cargo-update](https://crates.io/crates/cargo-update)

## Shared toolchains

By default the toolchains go to `$XDG_DATA_HOME/rustup`. If `/opt/rustup`
exists, the module uses that instead. It links `21-rustup-shared.conf` into
`environment.d` to set `RUSTUP_HOME`.

A user who cannot write to `/opt/rustup` skips the rustup steps but still gets
the cargo tools. That needs a system-wide `cargo`, like the one in the Arch
`rustup` package.

To start sharing:

1. Create the folder with read access for the sharing users. `GROUP` is a
   group that contains them:
   `sudo install -d -m 2750 -o "$USER" -g GROUP /opt/rustup`
2. Copy the contents of `$XDG_DATA_HOME/rustup` into it, then check that the
   folder kept its mode.
3. Each user runs `pont -fx rust`, or `rustup_shared=1 pont -fxS rust` to only
   link the file.

To stop sharing, each user runs `pont -r rust`. Once the owner has removed the
folder, each user runs `pont -fx rust` to use `$XDG_DATA_HOME/rustup` again.

If the owner removes the folder first, the leftover link keeps `RUSTUP_HOME`
set to the missing folder. The module warns about it until `pont -r rust`
removes the link.
