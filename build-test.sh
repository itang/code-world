#!/bin/bash

DIR=`pwd`
#PS="$DIR/rust-master-tutorial
#    $DIR/rust-additional
#    $DIR/rust-for-rubyists
#    $DIR/snippets/delay
#    $DIR/snippets/goroutines_rusttasks
#    $DIR/tiny-tools/rust-time"

PS=`find . -type f -name Makefile`
echo $PS

function makep() {
    echo "cd $1 && make";
    (cd $1; make);
    echo;
}

for p in $PS; do
    makep $(dirname $p);
done

echo Finished!
