

help: _help_

_help_:
	@echo make fmt - format code
	@echo make test - run tests
	@echo make build - build binary
	@echo make audit - run audit
	@echo make coverage - run coverage

.PHONY: help fmt test build audit expand coverage

fmt:
	cargo fmt

test:
	cargo test

build:
	cargo build --release

audit:
	cargo audit

coverage:
	cargo tarpaulin --ignore-tests --out Xml

expand:
	cargo expand
