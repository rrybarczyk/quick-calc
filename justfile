test: 
    cargo watch --clear --exec test

check: 
    cargo watch --clear --exec check

test-print:
    cargo test -- --nocapture

build: 
	cargo build --release

run: 
	cargo build --release
	./target/release/qcal format 3
	./target/release/qcal format 3 6

publish:
	cargo build
	cargo publish
