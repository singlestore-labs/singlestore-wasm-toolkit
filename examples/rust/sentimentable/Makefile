MODULE := sentimentable

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
	    -e '[{"compound": 0.5573704017131537, "positive": 0.5454545454545455, "negative": 0.0, "neutral": 0.4545454545454546 }]' \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm sentimentable\
            "it is a fantastic day"
	@echo PASS
	writ \
	    -e '[{"compound": -0.5423261445466404, "positive": 0.0, "negative": 0.5384615384615384, "neutral": 0.46153846153846156 }]' \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm sentimentable\
            "it is a bad day"
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

