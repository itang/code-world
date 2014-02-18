OUT_DIR=.tmp

files=\
	conditions.rs \
	iteration.rs \
	lifetimes.rs \
	pointers.rs \
	tasks.rs \
	$(wildcard outer/*.rs) \
	$(wildcard inside/*.rs)

default: build clean

build:
	mkdir -p $(OUT_DIR)
	for item in $(files); \
	do \
		echo Compiling $$item; \
		rustc $$item --out-dir $(OUT_DIR) || exit; \
	done

clean:
	rm -rf  $(OUT_DIR)
