/**
 * this script is an example of importing the generated `.js` module into another
 * JavaScript module (which is this file). The generated `.js` module in turns
 * import the `.wasm` binary and eventually everything we expose in our rust code
 * will be available in the `import`ed JS library. See more below.
 */

import * as lib from "../dist-node/cosmos_tools.js";

// console.log(lib);
// lib.unsupportedMethod1();
// lib.unsupportedMethod2();
// lib.main();

lib.setup(process.env);

console.log("script finished");