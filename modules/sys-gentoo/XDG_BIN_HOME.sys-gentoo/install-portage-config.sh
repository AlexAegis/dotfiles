#!/bin/sh

# get last segment
source_folder=${1:-"./resources/portage"}
source_folder_name="${source_folder%%/}"
source_folder_name="${source_folder_name##*/}"
target_folder="/etc/portage/"

if [ ! -d "$source_folder" ]; then
	echo "Folder $source_folder_name does not exist."
	exit 1
elif [ ! "$source_folder_name" = 'portage' ]; then
	echo "Source folder must be named portage to avoid installing unwanted\
	files."
	exit 1
fi

(
	cd "$source_folder" || exit 1
	mkdir -p "$target_folder"
	cp --recursive --update --parents --backup=numbered "." "$target_folder"
)


