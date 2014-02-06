#!/bin/bash

cd .tmp
script -t 2> timing.log -a output.session

ls -l
ruby -v

exit

scriptreplay timing.log output.session

cd .tmp