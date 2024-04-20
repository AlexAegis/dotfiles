#!/bin/bash

# Initialize database
su -l postgres -c "initdb --locale=en_US.UTF-8 --encoding=UTF8 -D '/var/lib/postgres/data'"

# Enable stronger passwords
appendabsent 'password_encryption = scram-sha-256' /var/lib/postgres/data/postgresql.conf
