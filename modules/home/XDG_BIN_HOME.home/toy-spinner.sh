#!/bin/sh

echo "Show spinner"

# The command you are waiting on goes between the ( ) here
# The example below returns a non zero return code

(sleep 20 ; /bin/false) &

pid=$! ; i=0
while ps -a | awk '{print $1}' | grep -q "${pid}"
do
    c=$((i % 4))
    case ${c} in
       0) printf '/' ;;
       1) printf '-' ;;
       2) printf '\\ \b' ;;
       3) printf '|' ;;
    esac
    i=$((i + 1))
    # change the speed of the spinner by altering the 1 below
    sleep 1
    printf '\b'
done

# Collect the return code from the background process

wait ${pid}
ret=$?

# You can report on any errors due to a non zero return code here

exit ${ret}
