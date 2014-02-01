#!/bin/bash

function cout() {
    echo "*out:\n`cat .tmp/out.txt`";
}

# ###################################
rm -rf .tmp
mkdir .tmp
echo "this is a sample text 1" > .tmp/temp.txt

content=`cat .tmp/temp.txt`
echo $content
echo
echo "This is a sample text 2" >> .tmp/temp.txt
echo "`cat .tmp/temp.txt`"

ls + > .tmp/out.txt
cout
ls + 2>.tmp/out.txt
cout

ls + 2>&1 .tmp/out.txt
cout

ls -l | tee .tmp/out.txt | cat -n
cout
