#!/bin/sh

echo "Show spinner"

# The command you are waiting on goes between the ( ) here
# The example below returns a non zero return code

(sleep 20 ; /bin/false) &

pid=$! ; i=0
while ps -a | awk '{print $1}' | grep -q "${pid}"
do
    c=`expr ${i} % 4`
    case ${c} in
       0) echo "/\c" ;;
       1) echo "-\c" ;;
       2) echo "\\ \b\c" ;;
       3) echo "|\c" ;;
    esac
    i=`expr ${i} + 1`
    # change the speed of the spinner by altering the 1 below
    sleep 1
    echo "\b\c"
done

# Collect the return code from the background process

wait ${pid}
ret=$?

# You can report on any errors due to a non zero return code here

exit ${ret}
