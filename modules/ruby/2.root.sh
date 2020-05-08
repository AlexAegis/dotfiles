#!/bin/bash

usermod -a -G rvm "$USER"



rvm get stable
rvm use ruby --install --default
