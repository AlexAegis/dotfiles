#!/bin/sh

# Set up a samba password for the user if not already

reload_service smb.service
reload_service nmb.service
