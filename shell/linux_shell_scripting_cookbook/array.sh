#!/bin/bash

array_var=(1 2 3 4 5 6)
echo ${array_var[0]}

arr[0]="t1"
arr[1]="t2"
echo ${arr[1]}

index=0
echo ${arr[$index]}

declare -A ass_array
ass_array=([index1]=val1 [index2]=val2)

ass_array[index3]=val3

echo ${ass_array["index2"]}
echo ${ass_array[index3]}

echo ${!ass_array[*]}
echo ${ass_array[*]}

