MODULE := split

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
	    -e '[{"str": "hello", "idx": 0}, {"str": "there", "idx": 6}, {"str": "people", "idx": 12}]' \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm split-str \
            "hello there people" " "
	@echo PASS
	writ \
	    -e '[{"str": "hello", "idx": 0}]' \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm split-str \
            "hello" " "
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

