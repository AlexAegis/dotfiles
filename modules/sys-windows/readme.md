# Windows

## Timezone settings

To dual boot with Windows it is recommended to configure Windows to use UTC,
rather than Linux to use localtime.

> 1.user.ps1 sets the registry for you

### Troubleshooting

Setting the registry entry to use UTC is not enough if you already have a
timezone selected because now it would still result in an offset, just in the
other direction. So make sure to set both `Set time automatically` and
`Set timezone automatically` to on in the Date & time settings page, then hit
Sync Now.

> The timezone field will go grey.
