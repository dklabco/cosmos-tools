/**
 * at wirting time, the only way to successfuly import the wasm-pack generated
 * JS module is through the Deno std/node module, as seen below
 */

import { createRequire } from "https://deno.land/std@0.138.0/node/module.ts";

// import.meta.url will be the location of "this" module (like `__filename` in
// Node.js), and then serve as the root for your "package", where the
// `package.json` is expected to be, and where the `node_modules` will be used
// for resolution of packages.
// console.log("import.meta.url", import.meta.url);
const require = createRequire(import.meta.url);

const lib = require("../dist-node/cosmos_tools.js");

// console.log(lib);
// lib.unsupportedMethod1();
// lib.unsupportedMethod2();
// lib.main();

lib.setup(Deno.env.toObject());

console.log("script finished");