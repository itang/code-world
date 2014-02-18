files=\
	tutorial-sources.rs

default: run clean

build: $(files)
	mkdir -p .tmp
	rustc tutorial-sources.rs --out-dir=.tmp

test: $(files)
	mkdir -p .tmp
	rustc tutorial-sources.rs --test --out-dir=.tmp

run: build
	./.tmp/tutorial-sources

clean:
	rm -rf .tmp
