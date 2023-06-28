MODULE := usergenerator

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

# This function produces random output, so we can't check it against a
# standard.  Instead, just verify that it runs without error.
.PHONY: test
test:
	writ --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm gen-users 3
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

