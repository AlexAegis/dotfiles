# fehbg

Using feh as a simple wallpaper provider

## Temporary wallpaper

Set the WALLPAPER variable in systemd then restart the servive.
You have to trigger a deamon reload to change it back.

```sh
systemctl --user set-environment WALLPAPER="$XDG_PICTURES_DIR/wallpaper.jpg"
systemctl --user unset-environment WALLPAPER
systemctl --user daemon-reload
```
