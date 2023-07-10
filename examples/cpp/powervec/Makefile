SHELL := bash

.PHONY: release
release: DBGFLAGS = -O3
release: module loader

.PHONY: debug
debug: DBGFLAGS = -g
debug: module loader

.PHONY: loader
loader: load_extension.sql

.PHONY: module
module: extension.wasm

extension.wasm: extension.c
	@if [ -z "$$WASI_SDK_PATH" ] ; then \
		echo "Please set the WASI_SDK_PATH environment variable to the location of the WASI SDK."; \
		exit 1; \
	fi; \
	export CLANG="$$WASI_SDK_PATH/bin/clang"; \
	if [ ! -f "$$CLANG" ] ; then \
		echo "No clang compiler was found at $$CLANG.  Aborting."; \
		exit 1; \
	fi; \
	export CLANGTGT=`$$CLANG --version | grep Target: | awk '{print $$2;}'`; \
	if [ "$$CLANGTGT" != "wasm32-unknown-wasi" ] ; then \
		echo "The clang at $$CLANG does not support the WASI SDK.  Aborting."; \
		exit 1; \
	fi; \
	$$CLANG \
		${DBGFLAGS} \
		-fno-exceptions \
 		--target=wasm32-unknown-wasi \
 		-mexec-model=reactor \
 		-I. \
 		-Icommon \
 		-Itheta-sketch \
 		-o extension.wasm \
 		extension.c extension_impl.c

extension.c: extension.wit
	wit-bindgen c --export extension.wit

load_extension.sql: create_loader.sh
	./create_loader.sh

.PHONY: clean
clean:
	@rm -f extension.wasm load_extension.sql

.PHONY: distclean
distclean: clean
	@rm -f extension.c extension.h

.PHONY: test
	test: debug
	writ-docker --expect '[]' extension.wit extension.wasm extension_vector_pow_f64 '[]'
