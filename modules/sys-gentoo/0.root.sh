#!/bin/sh

# Make/convert default config folders
folderify() {
	if [ -f "$1" ]; then
		filename="${1##*/}"
		tmp_name="_$(date +%s)_temp_file_$filename"
		mv "$1" "$HOME/$tmp_name"
		mkdir -p "$1"
		mv "$HOME/$tmp_name" "$1/$filename"
	else
		mkdir -p "$1"
	fi
}

# Portage can either read the content of these files or if they are folders
# then the content of all the files inside them. I prefer them to be folders
# so other modules can extend them with symlinks. But by the time this scipts
# tries to create these folders portage might have created these.
# So I just wrap them in a folder.
folderify /etc/portage/package.accept_keywords
folderify /etc/portage/package.license
folderify /etc/portage/package.mask
folderify /etc/portage/package.unmask
folderify /etc/portage/package.use

# Install make conf and other default portage files
# shellcheck disable=SC1091
./XDG_BIN_HOME.sys-gentoo/install-portage-config.sh

echo "*/* ~$(portageq envvar ARCH)" > \
	/etc/portage/package.accept_keywords/10-architecture
