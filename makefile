simple: 
	cargo run --release --bin simple-std-socket -- client | cargo run --release --bin simple-std-socket -- server

simple-udp: 
	cargo run --release --bin simple-udp-socket -- client & cargo run --release --bin simple-udp-socket -- server