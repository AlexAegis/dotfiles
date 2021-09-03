# TTY Screensavers

Some terminal programs that look neat when running full screen

## Disclaimer about Raspberry Pi's

When using a pi, by default it logs to console and that can interfere with
programs running as screensavers. To make them disappear add `quiet` to
`/boot/cmdline.txt`

## Disclaimer about using TMUX

If you try running multiple with `tmux` be aware that it just exposes root
prompts. If you use this as a `tty` replacement anyone would get root access
to your server who can plug a keyboard into it.
