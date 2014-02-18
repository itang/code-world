default: build test clean

prepare:
	mkdir -p .tmp

test: prepare
	go test -v ./...
	rustc --test -o .tmp/test-rust rust.rs
	.tmp/test-rust

build: prepare
	go build -o .tmp/run-go go.go
	rustc -o .tmp/run-rust rust.rs

clean:
	rm -rf .tmp
