# [KDE](https://kde.org/plasma-desktop)

> [Arch Wiki](https://wiki.archlinux.org/index.php/KDE)

## [KDE Wallet](https://wiki.archlinux.org/title/KDE_Wallet)

KDE Keychain solution

## Troubleshooting

### Missing Applications

```sh
update-desktop-database ~/.local/share/applications -v
```

### Missing Applications (Older note)

To regenerate the settings run these:

```sh
cp /etc/xdg/menus/applications.menu ~/.config/menus/
kbuildsycoca6 --noincremental
```
