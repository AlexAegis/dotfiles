#!/bin/sh

if command -v psql >/dev/null; then
	printf "Are you sure you want to delete the 'giteadb' database and 'gitea' role from postgresql? [Y]: "
	read confirm
	echo ""

	if [ "$confirm" = "Y" ]; then
		psql -U postgres postgres -c "DROP DATABASE giteadb;"
		psql -U postgres postgres -c "DROP ROLE gitea;"
		echo "The 'giteadb' database and 'gitea' role was dropped."
	else
		echo "The 'giteadb' database and 'gitea' role was NOT dropped."
	fi
else
	echo "postgresql not installed, not trying to drop the 'giteadb' database and 'gitea' role."
fi
