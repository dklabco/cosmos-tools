/**
 * this script is an example of importing the generated `.js` module into another
 * JavaScript module (which is this file). The generated `.js` module in turns
 * import the `.wasm` binary and eventually everything we expose in our rust code
 * will be available in the `import`ed JS library. See more below.
 */

import * as lib from "../pkg/cosmos_tools.js";

console.log(lib); // prints some of the inner details of the imported js module

lib.bark_at("Cosmos"); // prints "wuf, Cosmos"