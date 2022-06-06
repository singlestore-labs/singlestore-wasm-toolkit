all: power.wasm

power.wasm:
	/opt/wasi-sdk-14.0/bin/clang        \
	    --target=wasm32-unknown-wasi    \
	    -mexec-model=reactor            \
	    -s                              \
	    -I.                             \
	    -o power.wasm                   \
	    power.c

clean:
	@rm -f power.wasm

.PHONY: all clean

