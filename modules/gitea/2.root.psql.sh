#!/bin/sh


gitea_postgresql_password=$(read_password "Please enter a password for the gitea role in postgresql")
echo "Creating gitea postgres role..."
psql -U postgres postgres -c "CREATE ROLE gitea WITH LOGIN PASSWORD '$gitea_postgresql_password';"

echo "Creating gitea postgres database..."
psql -U postgres postgres -c "CREATE DATABASE giteadb WITH OWNER gitea TEMPLATE template0 ENCODING UTF8 LC_COLLATE 'en_US.UTF-8' LC_CTYPE 'en_US.UTF-8';"
