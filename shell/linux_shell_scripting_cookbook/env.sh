#!/bin/bash

bash_pids=`pgrep bash`
echo bash_pids $bash_pids

export a=100
export a=$a:1000
echo $a

echo a.length ${#a}

#echo PATH $PATH
echo "PWD    $PWD"
echo "USER   $USER"
echo "HOME   $HOME"
echo "UID    $UID"
echo "SHELL  $SHELL"
#sudo cat /proc/1/environ

echo SHELL $SHELL
echo '$0' $0
echo '$1' $1
echo "`ps -ef | grep sh`"
echo `pgrep bash -l`
echo pid $$

echo
echo '$@:' "$@"
