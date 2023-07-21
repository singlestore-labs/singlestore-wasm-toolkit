MODULE := xpath

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
	writ \
	    -e '"hello there"' \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm eval-xpath \
            "<a><b>hello there</b></a>" "/a/b"
	@echo PASS
	writ \
	    -e '["hello there", "hello there"]' \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm eval-xpaths \
            "<a><b>hello there</b><b>hello there</b></a>" "/a/b"
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

