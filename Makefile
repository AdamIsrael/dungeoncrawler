build:
	cargo build

clean:
	rm -rf dist
	cargo clean

dist: release
	mkdir -p dist/resources
	cp target/release/dungeoncrawl dist
	cp -R resources/* dist/resources
	
release:
	cargo build --release

test:
	cargo test
	cargo fmt
	cargo clippy -- -D warnings

all: build test

help:
	@echo "usage: make $(prog) [debug=1]"