files=\
	$(wildcard ./*.rs)

default: build clean

build: $(files)
	mkdir -p .tmp
	for f in $(files); \
	do \
	    echo Compile $$f; \
	    rustc $$f --out-dir .tmp || exit; \
	done

clean:
	rm -rf .tmp
