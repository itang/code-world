#!/bin/bash

## for var in list; 
## do

## commonds;

## done

### list can be string, or a sequence.

echo {1..50}
echo {a..z}
echo {A..Z}

for i in {0..10} # 0 1 2...10
do 
echo $i
done

for((i=0;i<10;i++)) {
    echo $i;
}
