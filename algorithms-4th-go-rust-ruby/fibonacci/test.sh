#!/bin/bash

# prepare
mkdir .tmp

# test
echo test go
go test ./...

echo test rust
rustc --test -o ./.tmp/fib-rust-test.bin fib.rs
./.tmp/fib-rust-test.bin

echo "*******************************************************************"

# run
echo "===="
echo run fib.go
go build -o ./.tmp/fib-go.bin fib.go
time ./.tmp/fib-go.bin

echo "===="
echo run fib.rs
rustc -o ./.tmp/fib-rust.bin fib.rs
time ./.tmp/fib-rust.bin

echo "===="
echo run fib.rb
time ruby fib.rb

# clean
rm -rf .tmp
