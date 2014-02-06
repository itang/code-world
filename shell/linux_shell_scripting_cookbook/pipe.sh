#!/bin/bash

function cout(){
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

ls | cat -n > .tmp/out.txt
cout

cmd_output=$(ls -l | cat -n)
echo "$cmd_output"

cmd_output=`echo $USER | cat -n`
echo ""
echo "$cmd_output"
cmd_output=`ls + | cat -n`
if [ "$cmd_output" == "" ]; then
    echo "NO"
fi

# () for new shell process

pwd;
mkdir .tmp;
(cd .tmp; pwd; ls;);

pwd;

# use "" reserve \n
echo "$(ls -l)"