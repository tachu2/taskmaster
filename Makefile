.PHONEY all:
all: build

.PHONEY build:
build:
	cargo build

.PHONEY run:
run:
	cargo run

.PHONEY fmt:
fmt:
	cargo fmt

.PHONEY lint:
	cargo clippy

.PHONEY test:
test:
	cargo test
