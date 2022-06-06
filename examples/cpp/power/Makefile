all: power.wasm

power.wasm:
	clang     						    \
	    --target=wasm32-unknown-wasi    \
	    -mexec-model=reactor            \
	    -s                              \
	    -I.                             \
	    -o power.wasm                   \
	    power.c

clean:
	@rm -f power.wasm

.PHONY: all clean

