.PHONY: build release run check test fmt fmt-check clippy lint clean install help

.DEFAULT_GOAL := help

CARGO ?= cargo

build:
	$(CARGO) build

release:
	$(CARGO) build --release

run:
	$(CARGO) run -- $(ARGS)

check:
	$(CARGO) check

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt

fmt-check:
	$(CARGO) fmt -- --check

clippy:
	$(CARGO) clippy -- -D warnings

lint: fmt-check clippy

clean:
	$(CARGO) clean

install:
	$(CARGO) install --path .

help:
	@echo "wikid development commands:"
	@echo ""
	@echo "  make build        Build debug binary"
	@echo "  make release      Build optimized release"
	@echo "  make run          Run wikid (use ARGS=\"...\" to pass arguments)"
	@echo "  make check        Run cargo check"
	@echo "  make test         Run unit tests"
	@echo "  make fmt          Format source code"
	@echo "  make fmt-check    Check formatting without applying"
	@echo "  make clippy       Run clippy with warnings denied"
	@echo "  make lint         Run formatting + clippy checks"
	@echo "  make clean        Remove build artifacts"
	@echo "  make install      Install wikid locally via cargo install"
	@echo "  make help         Show this help"
