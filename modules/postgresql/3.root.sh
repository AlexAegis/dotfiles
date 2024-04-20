#!/bin/sh

# TODO: Fix this, it stalls, also fix it in the gitea module
# TODO: check if already present, skip if it is
gitea_postgresql_password=$(read_password "Please enter a password for the gitea role in postgresql")
echo "Creating root postgres role..."
psql -U postgres postgres -c "CREATE ROLE root WITH LOGIN PASSWORD '$gitea_postgresql_password';"
