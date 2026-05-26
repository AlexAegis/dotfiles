# [KDE](https://kde.org/plasma-desktop)

> [Arch Wiki](https://wiki.archlinux.org/index.php/KDE)

## [KDE Wallet](https://wiki.archlinux.org/title/KDE_Wallet)

KDE Keychain solution

## Troubleshooting

### Missing Applications

These are now run automatically by the KDE module init script, but can also be
run manually when troubleshooting a live session:

```sh
update-desktop-database ~/.local/share/applications -v
```

### Missing Applications (Older note)

To regenerate the settings run these:

```sh
cp /etc/xdg/menus/plasma-applications.menu ~/.config/menus/applications.menu
kbuildsycoca6 --noincremental
```
