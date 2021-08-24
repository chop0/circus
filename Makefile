all:
	cargo build --release
	cp target/release/acrobatics docker/dist/circus
	strip docker/dist/circus
