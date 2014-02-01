#!/bin/bash

echo "`date +%s`"
echo "`date "+%d %B %Y"`"

start=$(date +%s)

sleep 1

end=$(date +%s)
diff=$(( end - start ))
echo Time  taken to execute commands is $diff seconds.

echo -n Count:
tput sc 

count=0
while true; do
    if [ $count -lt 10 ]; then
        let count++
        sleep 1
        tput rc
        tput ed 
        echo -n $count
    else exit 0
    fi
done

echo