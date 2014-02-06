#!/bin/bash

## if condition;
## then
## commonds;
## fi

if true; then
    echo "if then "
elif false; then
    echo "elif"
else
    echo "else"
fi

[ true ] && echo "true";
[ "$a" != "" ] || echo "false";

count=1
if [ $count -eq 1 ]; then
    echo "count is 1"
fi

a=$[1+1];
echo $a

if [ -f "if.sh" ]; then
    echo "exists if.sh"
fi

if [ -f "if1.sh" ]; then
    echo "exists if1.sh"
else 
    echo "not exists if1.sh"
fi

if [ -x "array.sh" ]; then
    echo "array.sh can execute"
fi

a="."
if [ -d $a ];then
    echo $a is dir
fi

if [ -e "if.sh" ]; then
    echo "if.sh"
fi 

if [ -r "if.sh" ]; then
    echo "if.sh readonly"
fi


if [ -w "if.sh" ]; then
    echo "if.sh writeable"
fi

if [ -L "../lssc" ]; then
    echo "../lssc link 
`ls -l ../lssc`"
fi

## string compare
name="itang"
if [[ $name = "itang" ]]; then
    echo "name is itang"
fi

if [[ $name != 'tqibm' ]];then
    echo "name != 'tqibm'"
fi

if [[ $name > "itang1" ]];then
    echo ">"
else echo "<"
fi

if [[ -z $age ]]; then
    echo "age is empty "
fi

if [[ -n $count ]]; then
    echo "$count not empty"
else echo "$count is empty"
fi

if [[ -n "aa" ]] && [[ -z "" ]]; then 
    echo "ok"
fi