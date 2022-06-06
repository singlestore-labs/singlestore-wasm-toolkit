all: split.wasm

split.wasm:
	/opt/wasi-sdk-14.0/bin/clang++      \
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

