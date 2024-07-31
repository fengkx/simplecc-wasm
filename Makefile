.PONY: default

default: opt

rust:
	@npm run build:cargo
nodejs: rust
	@wasm-bindgen target/wasm32-unknown-unknown/release/simplecc_wasm.wasm --out-dir ./pkg/nodejs --target nodejs
web: rust
	@wasm-bindgen target/wasm32-unknown-unknown/release/simplecc_wasm.wasm --out-dir ./pkg/web --target web

deno: rust
	@wasm-bindgen target/wasm32-unknown-unknown/release/simplecc_wasm.wasm --out-dir ./pkg/deno --target deno

build: nodejs web deno

opt: build
	@wasm-opt -Oz -o pkg/web/simplecc_wasm_bg.owasm pkg/web/simplecc_wasm_bg.wasm
	@mv pkg/web/simplecc_wasm_bg.owasm pkg/web/simplecc_wasm_bg.wasm
	@wasm-opt -Oz -o pkg/nodejs/simplecc_wasm_bg.owasm pkg/nodejs/simplecc_wasm_bg.wasm
	@mv pkg/nodejs/simplecc_wasm_bg.owasm pkg/nodejs/simplecc_wasm_bg.wasm