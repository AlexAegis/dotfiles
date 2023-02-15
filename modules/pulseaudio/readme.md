# [PulseAudio](https://www.freedesktop.org/wiki/Software/PulseAudio/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/PulseAudio)

For audio drivers

> There are system specific settings for audio

## Config

> Try `alsamixer`

### List sinks

```sh
pactl list short sinks
```

### Set default sink

```sh
pacmd set-default-sink <ID>
```
