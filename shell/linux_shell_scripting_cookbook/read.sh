#!/bin/bash

## read -n number_of_chars variable_name
echo "read 2 char"
read -n 2 var 
echo ""
echo $var

echo "Enter password:"
read -s password
echo "password:$password"

read -p "Enter input:" var

echo $var

echo "read in timeout"
read -t 2 var
echo $var

echo "delim"
read -d ":" var
echo $var