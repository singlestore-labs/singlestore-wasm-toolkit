MODULE := unicode_normalizer

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
	# the ô character in the following tests are encoded in different ways
	# to test out normalization
	writ \
	    -e "Yôga" \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm normalize-unicode-nfd \
		"Yôga"
	@echo PASS
	writ \
	    -e "Yôga" \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm normalize-unicode-nfc \
		"Yôga"
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

