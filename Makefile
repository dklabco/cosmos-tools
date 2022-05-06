build:
	wasm-pack build --target nodejs

start:
	cd examples && npm start

# this throws on purpose
throw:
	cd examples && npm run throw