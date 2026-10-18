# [Rust](https://www.rust-lang.org/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/rust)\
> [Gentoo Wiki](https://wiki.gentoo.org/wiki/Project:Rust)

This module installs `rustup` with it the stable toolchain and some
must have crates such as [cargo-update](https://crates.io/crates/cargo-update)

Toolchains are in `$XDG_DATA_HOME/rustup`. Where `/opt/rustup` exists, the
module links `21-rustup-shared.conf` into `$XDG_CONFIG_HOME/environment.d`,
which sets `RUSTUP_HOME` to `/opt/rustup`. A link left after the folder is
removed keeps that setting, with a warning, until `pont -r rust` removes it.
Users who cannot write to `/opt/rustup` skip the rustup steps. They still get
the cargo tools where `cargo` is installed system-wide, as with the Arch
`rustup` package.

To share toolchains on a machine, create the folder with read access for the
users that share it, such as
`sudo install -d -m 2750 -o "$USER" -g GROUP /opt/rustup`, where `GROUP`
contains those users. Copy the contents of `$XDG_DATA_HOME/rustup` into it, and
check that the folder kept its mode. Then each user runs `pont -fx rust`, or
`rustup_shared=1 pont -fxS rust` to only link the file. To stop sharing, each
user runs `pont -r rust`. Then the owner removes the folder, and each user runs
`pont -fx rust`.
