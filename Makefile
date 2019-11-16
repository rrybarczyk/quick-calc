rdbg:
	cargo build
	./target/debug/qcal format 4
	./target/debug/qcal format 5 6
	./target/debug/qcal format 0xDEADBEEF 3 o24 b1101101
	./target/debug/qcal add 0xFF 30 o24 b111 
	# ./target/debug/qcal endian 55bd840a78798ad0da853f68974f3d183e2bd1db6a842c1feecf222a00000000
	# ./target/debug/qcal endian 29263ed541e0072216baa08ead2d787754cb882f573543a10000486e00000000
	# ./target/debug/qcal endian abcdef
	./target/debug/qcal sub 0xFF 30 o24 b111
	./target/debug/qcal mul 0xFF 30 o24 b111
	./target/debug/qcal div 0xFF 30 o24 b111
	# ./target/debug/qcal bytelen 0xFF 0xDEADBEEF
	# ./target/debug/qcal charlen 0xFF 0xDEADBEEF

testdbg:
	cargo test -- --nocapture
