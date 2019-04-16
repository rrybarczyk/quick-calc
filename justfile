check: 
    cargo watch --clear --exec check

test: 
    cargo watch --clear --exec test

build: 
	cargo build --release

run: 
	cargo build --release
	./target/release/qcal add 0xFF 30 o24 b111 
	./target/release/qcal sub 0xFF 30 o24 b111 
	./target/release/qcal mul 0xFF 30 o24 b111 
	./target/release/qcal div 0xFF 30 o24 b111 

help:
	cargo build --release
	./target/release/qcal --help
    

publish:
	cargo build
	cargo publish
