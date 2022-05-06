build:
	wasm-pack build --target nodejs

start:
	node userland/main.mjs

# this throws on purpose
throw:
	node --experimental-wasm-modules userland/main.direct.mjs