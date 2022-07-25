.PHONY: debug
debug: DBGFLAGS = -g
debug: power.wasm

.PHONY: release
release: power.wasm

power.wasm:
	clang                               \
	    $(DBGFLAGS)                     \
	    --target=wasm32-unknown-wasi    \
	    -mexec-model=reactor            \
	    -I.                             \
	    -o power.wasm                   \
		power_impl.c                    \
	    power.c

.PHONY: test
test: debug
	writ --expect 8 --wit power.wit power.wasm power-of 2 3
	@echo PASS
	writ --expect 1 --wit power.wit power.wasm power-of 2 0
	@echo PASS
	writ --expect 0 --wit power.wit power.wasm power-of 0 2
	@echo PASS

.PHONY: clean
clean:
	@rm -f power.wasm

