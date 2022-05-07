/**
 * this script is an example of importing the generated `.wasm` binary directly
 * into a JavaScript module (which is this file).
 * @NOTE THIS DOES NOT WORK - at least with the toolchain we have at writing time
 */

import * as lib from "../dist-node/cosmos_tools_bg.wasm";

console.log(lib);

console.log("script finished");