MODULE := power

.PHONY: debug
debug: $(eval TGT:=debug)
debug: wasm

.PHONY: release
release: $(eval TGT:=release)
release: RELFLAGS = --release
release: wasm

.PHONY: wasm
wasm:
	cargo wasi build --lib $(RELFLAGS)

.PHONY: test
test: debug
	writ --expect 8 --wit power.wit target/wasm32-wasi/debug/power.wasm power-of 2 3
	@echo PASS
	writ --expect 1 --wit power.wit target/wasm32-wasi/debug/power.wasm power-of 2 0
	@echo PASS
	writ --expect 0 --wit power.wit target/wasm32-wasi/debug/power.wasm power-of 0 2
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

