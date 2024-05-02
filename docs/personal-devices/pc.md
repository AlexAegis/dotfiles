# PC

Some notes specifically about my PC (AMD cpu, NVIDIA gpu) mainly
about what should be manually backed up

## /etc/mkinitcpio.conf

> Modules is configured for an NVIDIA gpu with proprietary drivers
> Hooks are configured to use systemd instead of the default udef for faster boot times.

```ini
MODULES=(nvidia nvidia_modeset nvidia_uvm nvidia_drm)
HOOKS=(base systemd autodetect microcode modconf kms keyboard sd-vconsole block sd-encrypt filesystems fsck)
```

Once you modify it, don't forget to regenerate your initramfs:

```sh
mkinitcpio
```

## /etc/fstab

Volume mount preferences and UUIDs

## /boot/EFI/refind/*

Refind config is PC specific with manual boot stanzas

## /etc/samba/smb.conf

Personal share locations
