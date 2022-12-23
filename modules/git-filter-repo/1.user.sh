#!/bin/sh


script_url="https://raw.githubusercontent.com/newren/git-filter-repo/main/git-filter-repo"
script_name="git-filter-repo"
cached_script="$XDG_BIN_HOME/$script_name"

if [ ! -f "$cached_script" ]|| timelock git_filter_repo; then
	wget "$script_url" -O "$cached_script"
	chmod +x "$cached_script"
	timelock git_filter_repo -f 'week'
else
	echo "git-filter-repo already downloaded or timelocked,\
 it unlocks at $(timelock -p git_filter_repo)"
fi

