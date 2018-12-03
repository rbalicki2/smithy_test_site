watch :
	cargo watch -x fmt -s 'make build' -w src -w ../smithy/

build :
	mkdir -p dist
	cp static/* dist/
	cargo +nightly build --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/smithy_test_site.wasm --out-dir ./dist