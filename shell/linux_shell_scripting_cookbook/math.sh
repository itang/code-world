#!/bin/bash

no1=4;
no2=5
no3=6;

result=$(( $no1+$no1 + no3 ))
echo $result

result=`expr 3 + 4`
echo $result

result=`echo "4 * 0.2" | bc`
echo $result

let a=no1+no2
echo $a

b=$[ 1 + 1 ]
echo $b