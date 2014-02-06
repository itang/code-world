#!/bin/bash

## cat file1 file2 fileN
# cat `ls`

if [ -f ".tmp" ]; then
    mkdir .tmp
fi

echo "world
	a

世界" > .tmp/hello.txt
echo "hello" | cat - .tmp/hello.txt

## cat -s file 压缩多个空白行为一行
cat -s .tmp/hello.txt

## \t 显示为 ^I
cat -T .tmp/hello.txt

## 显示行号
cat -n .tmp/hello.txt

