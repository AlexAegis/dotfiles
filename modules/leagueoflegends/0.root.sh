#!/bin/sh


snap refresh --candidate wine-platform-7-staging-core20
snap refresh --candidate wine-platform-5-staging
snap refresh --candidate wine-platform-runtime
snap install leagueoflegends --edge --devmode
