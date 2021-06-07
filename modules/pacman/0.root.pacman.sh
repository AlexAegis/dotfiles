#!/bin/sh

# It's extremely important
sed -i 's/#Color/Color\nILoveCandy/g' /etc/pacman.conf
# TODO: If the config editor script is ready, use it here

sed -i 's|#\[multilib\]|\[multilib\]\nInclude = /etc/pacman.d/mirrorlist|g' /etc/pacman.conf
sed -i 's/#ParallelDownloads.*/ParallelDownloads = 6/g' /etc/pacman.conf
# Some useful scripts like `paccache`
pacman -Syu --needed --noconfirm pacman-contrib

# aria2 https://wiki.archlinux.org/index.php/Pacman/Tips_and_tricks#aria2
# or just use powerpill, it uses aria2 internally
pacman -Syu --needed --noconfirm aria2
mkdir -p "/root/.aria2"
echo '# error handling
timeout=60
connect-timeout=30
max-tries=5
retry-wait=10
max-file-not-found=1

# downloading
split=3
max-connection-per-server=3
min-split-size=1M
max-concurrent-downloads=1
file-allocation=none
remote-time=true
conditional-get=true
no-netrc=true

# resuming
continue=true
allow-overwrite=true
always-resume=false

# proxy
#http-proxy=127.0.0.1:8080
#https-proxy=127.0.0.1:8080
#ftp-proxy=127.0.0.1:8080

# console
#quiet=true
console-log-level=warn
summary-interval=0
#enable-color=false
#human-readable=false
#show-console-readout=false
#truncate-console-readout=false

# logging
log-level=warn
log=/var/log/pacman-aria2.log
' >"/root/.aria2/pacman-aria2.conf"

# TODO: Use the option aware uncomment/set script when complete

# And put this: 'XferCommand = echo Downloading %u ... && /usr/bin/aria2c --conf-path=/root/.aria2/pacman-aria2.conf %u'
# into: /etc/pacman.conf

# Create pacman hook for automatic cache cleaning
mkdir -p '/etc/pacman.d/hooks'

echo '[Trigger]
Operation = Upgrade
Operation = Install
Operation = Remove
Type = Package
Target = *
[Action]
Description = Cleaning pacman cache...
When = PostTransaction
Exec = /usr/bin/paccache -r -k 2
' >'/etc/pacman.d/hooks/clean_package_cache.hook'
