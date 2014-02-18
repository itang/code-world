#!/bin/bash

vs="1.2.3-FINAL 1.2.3-SNAPSHOT"

for v in $vs; do
    case $v in 
        *SNAPSHOT) echo "SNAPSHOT";;
        *FINAL) echo "FINALL";;
    esac
done

## syntax:
# case in
#    xx) stat;;
#    yy) stat;;
# esac