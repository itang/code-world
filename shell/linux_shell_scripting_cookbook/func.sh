#!/bin/bash

function hello1() {
    echo 'hello1'
}

function hello2() {
    echo "hello2"
}

hello1; hello2;

function hello3() {
    name=$1
    echo "$@"; # "$1" "$2" "$n"
    echo "$*"; # "$1 $2 $3"
    echo "hello3 ${name}"
    return 1
}

hello3 itang tqibm live.tang;
echo $?; # return value
if [ $? -ne 1 ]; then
    echo "ss"
fi

# cmd arg1 arg2;