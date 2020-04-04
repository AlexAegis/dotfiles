#!/bin/sh

systemctl --user enable --now sxhkd.service
systemctl --user enable --now sxhkd.path
