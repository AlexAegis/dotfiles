#!/bin/sh

add-apt-repository -y ppa:longsleep/golang-backports
apt-get update -y
apt-get install -y golang-go

