#!/bin/sh

# Since the npmrc file can contain sensitive data like authentication tokens
# it is not directly traced via version control. Instead, the npm cli is used
# to manipulate it.

npm set fund false
npm set audit false
