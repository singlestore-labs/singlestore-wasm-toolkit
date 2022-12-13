MODULE := s2regex

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
		--expect '' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm capture \
		"empty" "empty"
	@echo PASS
	writ \
		--expect b \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm capture \
		"aabaaabbb" "(b+)"
	@echo PASS
	writ \
		--expect bbb \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm capture \
		"aabaaabbb" "(b{2,})"
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

