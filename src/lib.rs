extern crate console_error_panic_hook;

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}

// run automatically when imported into the userland
// run automatically once per thread, if WebAssembly threading is used;
// see https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/start.html
#[wasm_bindgen(start)]
pub fn main() {
    // this uses std::sync::Once internally, so no need to put it in there
    console_error_panic_hook::set_once();

    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        log("inited");
    });
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct JsEnv {
    pub COSMOS_TOOLS_HOME: String,
}

#[wasm_bindgen]
pub fn setup(js_env: &JsValue) {
    let parsed_env: JsEnv = js_env.into_serde().unwrap();
    log(&(json!(parsed_env)).to_string());
}

/*************************************************************************************************
 * below functions are for educational purposes only
 *************************************************************************************************/

#[wasm_bindgen(js_name = unsupportedMethod1)]
pub fn unsupported_method_1() {
    not_supported_env_vars();
}

#[wasm_bindgen(js_name = unsupportedMethod2)]
pub fn unsupported_method_2() {
    not_supported_current_exe();
}

/**
 * this is not supported
 */
fn not_supported_env_vars() {
    // for some reason, `catch_unwind` doesn't seem to work with wasm-pack; and in this case we
    // get a panic, then everything stops
    let op_result = panic::catch_unwind(|| {
        for (key, value) in env::vars() {
            let mut output = String::from("");
            output.push_str(&key.to_owned());
            output.push_str(" ");
            output.push_str(&value.to_owned());
            log(&output.to_owned())
        }
    });
    match op_result {
        Ok(_) => log("finished iterating through env vars"),
        Err(e) => log(&e.downcast::<String>().ok().unwrap_or_default()),
    }
}

/**
 * this is not supported
 */
fn not_supported_current_exe() {
    let current_exe = std::env::current_exe();
    match current_exe {
        Ok(exe_path) => log(exe_path.display().to_string().as_str()),
        Err(e) => log(e.to_string().as_str()), // prints "operation not supported on this platform"
    };
}
