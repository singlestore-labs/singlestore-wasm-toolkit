.PHONY: debug
debug: DBGFLAGS = -g
debug: split.wasm

.PHONY: release
release: split.wasm

split.wasm:
	clang++                             \
	    $(DBGFLAGS)                     \
	    -fno-exceptions                 \
	    --target=wasm32-unknown-wasi    \
	    -mexec-model=reactor            \
	    -I.                             \
	    -o split.wasm                   \
	    split.cpp

.PHONY: test
test: debug
	writ \
	    -e '[{"str": "hello", "idx": 0}, {"str": "there", "idx": 6}, {"str": "people", "idx": 12}]' \
            --wit split.wit split.wasm split-str "hello there people" " "
	@echo PASS
	writ \
	    -e '[{"str": "hello", "idx": 0}]' \
            --wit split.wit split.wasm split-str "hello" " "
	@echo PASS

.PHONY: clean
clean:
	@rm -f split.wasm

