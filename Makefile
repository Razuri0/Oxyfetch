compile:
	cargo build --release
run: compile
	clear
	cargo run
test:	
	clear
	cargo build
	cargo run

