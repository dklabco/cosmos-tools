build:
	wasm-pack build --target nodejs

start:
	node --experimental-wasm-modules userland/main.mjs