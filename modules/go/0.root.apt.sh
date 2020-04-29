#!/bin/sh

add-apt-repository -y ppa:longsleep/golang-backports
apt update -y
apt install -y golang-go

