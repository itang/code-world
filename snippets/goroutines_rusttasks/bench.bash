#!/bin/bash

RESULT_FILE=result.txt

function prepare() {
    make build;
}

function write_title() {
    echo >> $RESULT_FILE
    date >> $RESULT_FILE
    echo "===============================" >> $RESULT_FILE
}

function write_go() {
    go version >> $RESULT_FILE
    /usr/bin/time .tmp/run-go >> $RESULT_FILE 2>&1
}

function write_newline() {
    echo >> $RESULT_FILE
}

function write_rust() {
    rustc -v >> $RESULT_FILE
    /usr/bin/time .tmp/run-rust >> $RESULT_FILE 2>&1 
}

function do_clean() {
    echo ">clean..."
    rm -rf .tmp
}

function display_result() {
    echo; echo ">RESULT:"; echo "..................................................."
    tail -n 12 $RESULT_FILE
}

# main
prepare;
write_title;
echo ">run...";
write_go;
write_newline;
write_rust;
do_clean;
display_result;
