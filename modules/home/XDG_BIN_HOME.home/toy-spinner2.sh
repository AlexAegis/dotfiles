#!/bin/bash
# your real command here, instead of sleep
sleep 7 &
PID=$!
i=1
sp="/-\|"
echo -n ' '
while [ -d /proc/$PID ]
do
        sleep 0.2
    printf "\b%s" ${sp:i++%${#sp}:1}
done
