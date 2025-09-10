.PHONY: debug
debug: DBGFLAGS = -g
debug: split.wasm

.PHONY: release
release: split.wasm

split.wasm: gen
	clang++                             \
	    $(DBGFLAGS)                     \
	    -fno-exceptions                 \
	    --target=wasm32-unknown-wasi    \
	    -mexec-model=reactor            \
	    -I.                             \
	    -o split.wasm                   \
		split.cpp                       \
	    split_impl.cpp

.PHONY: test
test:
	writ \
	    -e '[{"str": "hello", "idx": 0}, {"str": "there", "idx": 6}, {"str": "people", "idx": 12}]' \
            --wit split.wit split.wasm split-str "hello there people" " "
	@echo PASS
	writ \
	    -e '[{"str": "hello", "idx": 0}]' \
            --wit split.wit split.wasm split-str "hello" " "
	@echo PASS

.PHONY: gen
gen:
	wit-bindgen c --export split.wit
	# temporary: https://github.com/bytecodealliance/wit-bindgen/issues/290
	sed "s:canonical_abi_realloc(NULL, 0, 1, ret->len:\(char \*\)canonical_abi_realloc(NULL, 0, 1, ret->len:g" split.c >split.cpp
	rm split.c

.PHONY: clean
clean:
	@rm -f split.wasm

