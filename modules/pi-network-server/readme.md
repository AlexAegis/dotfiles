# Pi Network Server

A module intended to configure an Pi server running Arch ARM.

## Routing

The associated modules assume the server is running on the fixed ip
`192.168.1.222`. This IP is aliased behind `pi` where it's possible (For
example in the `ssh` config of the [`pi-network-client`](../pi-network-client)
module).

> Check your local networks setting to set up a fixed ip agains the MAC
> address of the pi server.

## Bootconfig

You can add kernel flags in `/boot/cmdline.txt`

Add `root=/dev/sdXX` to point to your root partition.

And add `quiet` to disable console logging.
