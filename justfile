check: 
    cargo watch --clear --exec check

test: 
    cargo watch --clear --exec test

build: 
	cargo build --release

publish:
	cargo build
	cargo publish
