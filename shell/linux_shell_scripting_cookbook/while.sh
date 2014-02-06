#!/bin/bash

## while condition
## do
## commonds;
## done


while [[ $name != "itang" ]];
do
    echo $name
    echo enter itang
    read -p "Enter name:" name
done