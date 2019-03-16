watch :
	cargo watch -x fmt -s 'make build' -w src -w ../smithy/ -w ../basic_futures/

build :
	# rm -rf dist
	mkdir -p dist
	cp static/* dist/
	cargo +nightly build --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/smithy_test_site.wasm --out-dir ./dist

build_prod :
	rm -rf dist_prod
	mkdir -p dist
	cp static/* dist/
	cargo +nightly build --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/smithy_test_site.wasm --out-dir ./dist
	../binaryen/bin/wasm-opt -Oz -o dist/smithy_test_site.wasm dist/smithy_test_site_bg.wasm
	# NODE_ENV=production npm run webpack
	npm run webpack
	cp static/index.html dist_prod/

deploy :
	aws s3 sync dist_prod/ s3://smithy-test-site/ --cache-control max-age=0,no-cache --delete
	aws s3 cp dist_prod/*.wasm s3://smithy-test-site/ \
		--cache-control max-age=0,no-cache \
		--content-type application/wasm
	# aws cloudfront create-invalidation --distribution-id E3IDF5NLG30OGP --paths '/*'
