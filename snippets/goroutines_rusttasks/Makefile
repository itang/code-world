default: build test clean bench

prepare:
	mkdir -p .tmp

test: prepare
	go test -v ./...
	rustc --test -o .tmp/test-rust rust.rs
	.tmp/test-rust

build: prepare
	go build -o .tmp/run-go go.go
	rustc -o .tmp/run-rust rust.rs

bench: bench.bash
	bash ./bench.bash

clean:
	rm -rf .tmp
