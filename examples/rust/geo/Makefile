MODULE := geo

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
	    -e "POLYGON((1 0,1 1,0 1,0 0,1 0))" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-convexhull \
            "POLYGON((1 0,1 1,0 1,0 0,1 0))"
	@echo PASS
	writ \
	    -e "POINT(0.5 0.5)" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-centroid \
            "POLYGON((1 0,1 1,0 1,0 0,1 0))"
	@echo PASS
	writ \
	    -e "POINT(1 1)" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-centroid \
            "LINESTRING(0 0,1 1,2 2)"
	@echo PASS
	writ \
	    -e "POINT(1 1)" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-pointn \
            "LINESTRING(0 0,1 1,2 2)" "1"
	@echo PASS
	writ \
	    -e "POINT(2 2)" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-pointn \
            "LINESTRING(0 0,1 1,2 2)" "-1"
	@echo PASS
	writ \
	    -e "POINT(1 0.5)" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-closestpoint \
            "POLYGON((0 0,0 1,1 1,1 0,0 0))" "POINT(2 0.5)"
	@echo PASS
	writ \
	    -e "POINT(1 0.5)" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-closestpoint \
            "LINESTRING(0 0,0 1,1 1,1 0,0 0)" "POINT(0.6 0.5)"
	@echo PASS
	writ \
	    -e "POLYGON((0 0,0 1,1 1,1 0,0 0))" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-envelope \
            "POLYGON((0 0,0 1,1 1,1 0,0 0))"
	@echo PASS
	writ \
	    -e "true" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-intersects \
            "POLYGON((0 0,0 1,1 1,1 0,0 0))" "POINT(0 0)"
	@echo PASS
	writ \
	    -e "false" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-intersects \
            "POLYGON((0 0,0 1,1 1,1 0,0 0))" "POINT(2 0)"
	@echo PASS
	writ \
	    -e "POINT(1 0)" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-translate \
            "POINT(0 0)" "1" "0"
	@echo PASS
	writ \
	    -e "POINT(0.5403023058681398 0.8414709848078965)" \
            --wit $(MODULE).wit target/wasm32-wasi/debug/$(MODULE).wasm st-rotate \
            "POINT(1 0)" "1" "POINT(0 0)"
	@echo PASS

.PHONY: clean
clean:
	@cargo clean

