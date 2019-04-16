check: 
    cargo watch --clear --exec check

test: 
    cargo watch --clear --exec test

build: 
	cargo build --release

run: 
	cargo build --release
	./target/release/qcal add 128 16 2
	./target/release/qcal sub 128 16 2
	./target/release/qcal mul 128 16 2
	./target/release/qcal div 128 16 2

help:
	cargo build --release
	./target/release/qcal --help
    

publish:
	cargo build
	cargo publish
