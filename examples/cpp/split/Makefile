all: split.wasm

split.wasm:
	clang++                             \
	    -g                              \
	    -fno-exceptions                 \
	    --target=wasm32-unknown-wasi    \
	    -mexec-model=reactor            \
	    -I.                             \
	    -o split.wasm                   \
	    split.cpp

clean:
	@rm -f split.wasm

.PHONY: all clean

