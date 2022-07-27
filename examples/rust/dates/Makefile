MODULE := dates

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
	    -e "2022-07-30" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm next-saturday \
            "2022-07-23"
	cargo wasi test
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

