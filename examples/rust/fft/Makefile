MODULE := fft

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
	writ-docker --verbose \
		-e '[{"re":1.0,"im":2.0}]' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-process-forward \
		1 '[{"re":1.0,"im":2.0}]'
	@echo PASS
	writ-docker --verbose \
		-e '[{"re":1.0,"im":2.0}]' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-process-inverse \
		1 '[{"re":1.0,"im":2.0}]'
	@echo PASS
	writ-docker --verbose \
		-e '[{"re":1.0,"im":2.0}]' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-scalar-process-forward \
		1 '[{"re":1.0,"im":2.0}]'
	@echo PASS
	writ-docker --verbose \
		-e '[{"re":1.0,"im":2.0}]' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-scalar-process-inverse \
		1 '[{"re":1.0,"im":2.0}]'
	@echo PASS
	writ-docker --verbose \
		-e '[{"re":2.5,"im":4.5},{"re":-0.5,"im":-0.5}]' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-process-forward \
		2 '[{"re":1.0,"im":2.0},{"re":1.5,"im":2.5}]'
	@echo PASS
	writ-docker --verbose \
		-e '[{"re":2.5,"im":4.5},{"re":-0.5,"im":-0.5}]' \
		--wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-process-inverse \
		2 '[{"re":1.0,"im":2.0},{"re":1.5,"im":2.5}]'
	@echo PASS
.PHONY: clean
clean:
	@cargo clean


