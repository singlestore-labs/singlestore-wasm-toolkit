GEN = \
	Tutorial-WIT-Power.md \
	Tutorial-WIT-Split.md \
	Tutorial-Test-Power.md \
	Tutorial-Test-Split.md \
	Tutorial-Deploy-Power.md \
	Tutorial-Deploy-Split.md \
	Tutorial-Impl-Rust-Power.md \
	Tutorial-Impl-Rust-Split.md

.PHONY: all
all: $(GEN)

.PHONY: clean
clean:
	@rm -f $(GEN)

%.md : %.m4.md
	@echo "Generating $@..."
	@echo "<!-- GENERATED FILE; DO NOT CHANGE! -->" > $@
	@m4 -I./src/ $< >> $@
	@sed -i 's/\r$$//g' $@

