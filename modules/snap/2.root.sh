#!/bin/sh

# classic confinement requires snaps under /snap or symlink from /snap to /var/lib/snapd/snap
ln -s /var/lib/snapd/snap /snap
