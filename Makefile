build-nodejs:
	wasm-pack build --out-dir dist-node --target nodejs

build:
	$(MAKE) build-nodejs

start: export COSMOS_TOOLS_HOME = $(HOME)/.cosmos_tools
throw: export COSMOS_TOOLS_HOME = $(HOME)/.cosmos_tools
deno:  export COSMOS_TOOLS_HOME = $(HOME)/.cosmos_tools

start:
	cd examples && npm start

# this throws on purpose
throw:
	cd examples && npm run throw

# this requires Deno to have been installed (https://deno.land/#installation)
deno:
	deno run --allow-read --allow-env examples/main.deno.ts

################################################################################
# utilities and educational examples
#
std-env-vars:
	node examples/_lab_/std-env-vars.mjs
