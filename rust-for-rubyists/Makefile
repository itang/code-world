files=\
  $(wildcard ./*.rs)

OUT_DIR=.tmp

default: build clean

build: $(files)
	mkdir -p $(OUT_DIR)
	for f in $(files); \
	do \
	  echo Compile $$f; \
	  rustc $$f --out-dir=$(OUT_DIR); \
	done \

clean:
	rm -rf $(OUT_DIR)