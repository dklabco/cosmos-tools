import * as lib from "../../dist-node/cosmos_tools.js";

// this invokes `std::env::vars()` in the rust land, which panics ;(
// furthermore, `panic::catch_unwind` doesn't seem to be able to save the day
lib.unsupportedMethod1();