#!/bin/bash

data="name,sex,rellno,location"
oldIFS=$oldIFS
IFS=,
for item in $data; do
echo Item: $item
done
IFS=$oldIFS

line="root:x:0:0:root:/root:/bin/bash"
oldIFS=$IFS;
IFS=":"
count=0
for item in $line; do 

[ $count -eq 0 ] && user=$item;
[ $count -eq 6 ] && shell=$item;

let count++

done

IFS=$oldIFS;
echo $user\'s shell is $shell;