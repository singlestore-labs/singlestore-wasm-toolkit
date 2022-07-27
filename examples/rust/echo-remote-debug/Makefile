MODULE := echo

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
test:
	writ \
	    -e "hello there hello there" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm echo \
            "hello there"
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

