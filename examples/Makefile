SHELL := /bin/bash

debug: cpp-debug rust-debug
release: cpp-release rust-release

##############################################################################
# Build CPP
##############################################################################

.PHONY: cpp-debug
cpp-debug: TARGET=debug
cpp-debug: cpp

.PHONY: cpp-release
cpp-release: TARGET=release
cpp-release: cpp

.PHONY: cpp
cpp:
	@for D in `find cpp -maxdepth 1 -mindepth 1 -type d` ; do \
	    echo "=====================================================" ; \
	    echo "Building CPP example '`basename $$D`'" ; \
	    echo "=====================================================" ; \
		make -C $$D $(TARGET) test; \
	done

##############################################################################
# Build Rust
##############################################################################

.PHONY: rust-debug
rust-debug: rust

.PHONY: rust-release
rust-release: TARGET=release
rust-release: rust

.PHONY: rust
rust:
	@for D in `find rust -maxdepth 1 -mindepth 1 -type d` ; do \
	    echo "=====================================================" ; \
	    echo "Building Rust example '`basename $$D`'" ; \
	    echo "=====================================================" ; \
		make -C $$D $(TARGET) test; \
	done

##############################################################################
# Clean
##############################################################################

.PHONY: clean
clean: cpp-clean rust-clean

.PHONY: cpp-clean
cpp-clean:
	@for D in `find cpp -maxdepth 1 -mindepth 1 -type d` ; do \
	    echo "Cleaning CPP example '`basename $$D`'..." ; \
		make -C $$D clean ; \
	done

.PHONY: rust-clean
rust-clean:
	@for D in `find rust -maxdepth 1 -mindepth 1 -type d` ; do \
	    echo "Cleaning Rust example '`basename $$D`'..." ; \
		make -C $$D clean ; \
	done
