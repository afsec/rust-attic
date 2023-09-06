PRG_NAME=rtne-02

all: release

build-debug:
	cargo build

build-release:
	cargo build --release

debug: build-debug
	cp -v ./target/x86_64-unknown-linux-musl/debug/$(PRG_NAME) ./dist/

release: build-release
	mkdir -p ./dist
	cp -v ./target/x86_64-unknown-linux-musl/release/$(PRG_NAME) ./dist/
	strip ./dist/$(PRG_NAME)
	ls -lh ./dist

clean:
	rm -rf ./dist
	cargo clean
