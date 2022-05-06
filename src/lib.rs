use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}

#[wasm_bindgen]
pub fn bark_at(value: &str) {
    let mut output = String::from("wuf, ");
    output.push_str(value);
    log(&output);
}
