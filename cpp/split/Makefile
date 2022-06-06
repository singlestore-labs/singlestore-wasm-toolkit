all: split.wasm

split.wasm:
	clang++                             \
	    -fno-exceptions                 \
	    --target=wasm32-unknown-wasi    \
	    -mexec-model=reactor            \
	    -s                              \
	    -I.                             \
	    -o split.wasm                   \
	    split.cpp

clean:
	@rm -f split.wasm

.PHONY: all clean

