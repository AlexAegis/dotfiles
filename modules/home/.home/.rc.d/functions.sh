# Removes every word from $1 that occurs in $2
remove_words() {
	for a in $1; do
		local has=0
		for b in $2; do
			if [ "$a" = "$b" ]; then
				local has=1
			fi
		done
		if [ $has = 0 ]; then
			local res="$res $a"
		fi
	done

	echo $res
}
