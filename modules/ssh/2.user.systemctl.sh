#!/bin/sh

# Enable ssh-agent-add
systemctl enable --user --now ssh-agent.service
systemctl enable --user --now ssh-agent-add.service
