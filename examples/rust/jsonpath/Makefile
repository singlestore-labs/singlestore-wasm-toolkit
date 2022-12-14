MODULE := jsonpath

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
	# Note: Trying to avoid weird string encoding issues here
	@test "$(writ \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm eval_jsonpath \
		'{\"firstName\": \"John\"}' "$$.firstName")" = "$(echo John)" \
    	|| { echo Not equal; exit 2; } \
    	&& { echo PASS; }
	writ \
		--expect '42' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm eval_jsonpath \
		'{\"firstName\": \"John\", \"age\": 42}' "$$.age"
	@echo PASS
	writ \
		--expect '' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm eval_jsonpath \
		'{\"firstName\": \"John\"}' "$$.lastName"
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

